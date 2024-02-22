
#![allow(warnings)] use nix::libc::ARPD_FLUSH;
//Remove warning, be sure to remove this
use project_library::{AggregationStrategy, CountAggregateData, CountRawData, DataStrategy, TierData, process_tier, setup_my_app};
use std::{num, thread, usize};
use eframe::{egui, NativeOptions, App}; 
use egui::{Style, Visuals};
use egui_plot :: {BoxElem, BoxPlot, BoxSpread, Legend, Line, Plot};
use egui::{Vec2, CentralPanel};
use std::sync::{Arc, RwLock};
use crossbeam::channel;
use tokio::runtime::Runtime;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::time::{self, Duration, Interval};
use tokio::fs::File;
use tokio::sync::mpsc;
use std::env;
use std::time::{ Instant};
use rayon::{prelude::*, ThreadPool};
use std::sync::atomic::{AtomicBool, Ordering};
use std::error::Error;


struct MyApp {
    raw_data: Arc<RwLock<dyn DataStrategy + Send + Sync>>,  //'dyn' mean 'dynamic dispatch', specified for that instance. Allow polymorphism for that instance, don't need to know concrete type at compile time
    tiers: Vec<Arc<RwLock<TierData>>>,
    should_halt: Arc<AtomicBool>,
}

impl MyApp {

    pub fn new(
        raw_data: Arc<RwLock<dyn DataStrategy + Send + Sync>>, 
        tiers: Vec<Arc<RwLock<TierData>>>,
        should_halt: Arc<AtomicBool>,
        
    ) -> Self {
        Self { raw_data, tiers ,should_halt }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rd_sender, hd_receiver) = channel::unbounded();

    let (raw_data, tiers, should_halt, num_tiers)  = match setup_my_app() {
        Ok((raw_data, tiers, should_halt, num_tiers)) => (raw_data, tiers, should_halt, num_tiers),
        Err(e) => {
            eprintln!("{}", e);
            //return Err(Box::new(e));  // Early return for error case
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
        }
    };

    let my_app = MyApp::new(raw_data, tiers, should_halt);
    
    let rt = Runtime::new().unwrap();
    let should_halt_clone = my_app.should_halt.clone();
    let raw_data_thread = my_app.raw_data.clone();

    
    rt.spawn(async move {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();
        loop {
            if should_halt_clone.load(Ordering::SeqCst) {
                break; // Exit the loop if the atomic bool is true
            }
            tokio::select! {
                line = lines.next_line() => {
                    if let Ok(Some(line)) = line {
                        raw_data_thread.write().unwrap().append_str(line);
                        if let Some(cut_index) = raw_data_thread.write().unwrap().check_cut() {
                            rd_sender.send(cut_index).unwrap();
                        }

                    } else {
                        break;
                    }
                }, 
            }
        }
    });

    let aggregate_thread_raw_data_accessor = my_app.raw_data.clone();
    let initial_tier_accessor = my_app.tiers[0].clone();
    
    thread::spawn(move || {
        let mut chunk: Vec<[f64;2]>;
        let mut objective_length = 0;
        let mut aggregated_raw_data ; 
        for message in hd_receiver {

            {
                let mut aggregate_thread_raw_data_accessor_lock = aggregate_thread_raw_data_accessor.write().unwrap();
                chunk = aggregate_thread_raw_data_accessor_lock.get_chunk(message);
                aggregated_raw_data = aggregate_thread_raw_data_accessor_lock.append_chunk_aggregate_statistics(chunk);
                aggregate_thread_raw_data_accessor_lock.remove_chunk(7);
            }

            {
                let mut initial_tier_lock = initial_tier_accessor.write().unwrap();
                let length = initial_tier_lock.x_stats.len() -1 ;
                initial_tier_lock.x_stats[length] = aggregated_raw_data.2;
                initial_tier_lock.y_stats[length] = aggregated_raw_data.3;

                initial_tier_lock.x_stats.push(aggregated_raw_data.0);
                initial_tier_lock.y_stats.push(aggregated_raw_data.1);
            }
            
        }   
    });

    
    //rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();    



    let tier_vector = my_app.tiers.clone();
    let num_tiers = tier_vector.len();
    let catch_all_tier = tier_vector[num_tiers-1].clone(); //correctly gets the catch all tier, have to minus one since len not 0 indexed 

    //always have to drain final catch all vector
    catch_all_tier.write().unwrap().x_stats.drain(0..1);
    catch_all_tier.write().unwrap().y_stats.drain(0..1);
    
    let tier_check_cut_loop = thread::spawn(move || { 
        let mut merged_CA_last_x_element;
        let mut merged_CA_last_y_element;

        
        loop {
            //will break when only one tier, however this is an edge case, the Catch All only edge case
            for tier in 0..=(num_tiers-2) {  //only testing on first tier, initial tier, for now

                //println!("For tier {} the condition is {}", tier, tier_vector[tier].read().unwrap().condition);
                
                
                if tier_vector[tier].read().unwrap().x_stats.len() == tier_vector[tier].read().unwrap().condition {
                    //println!("\nTier {}", tier);
                    print!("{:?}", tier_vector[tier].read().unwrap().print_x_means("Before"));
                    process_tier(&tier_vector[tier], &tier_vector[tier+1], 7);
                    //print!("{:?}\n", tier_vector[tier].read().unwrap().print_x_means("After"));
                }
                
            } 
            thread::sleep(Duration::from_millis(1));

            let mut catch_all_tier_write_lock = catch_all_tier.write().unwrap();


            if catch_all_tier_write_lock.x_stats.len() == catch_all_tier_write_lock.condition {
                
                merged_CA_last_x_element = catch_all_tier_write_lock.merge_final_tier_vector_bins(3, true);
                merged_CA_last_y_element = catch_all_tier_write_lock.merge_final_tier_vector_bins(3, false);
                println!("Got the point {:?}", merged_CA_last_x_element);

                let mut tier_vector_write_lock = tier_vector[num_tiers-2].write().unwrap();

                println!("The first elem of t2 was {:?}", tier_vector_write_lock.x_stats[0]);
                tier_vector_write_lock.x_stats[0] = merged_CA_last_x_element;
                tier_vector_write_lock.y_stats[0] = merged_CA_last_y_element;
                println!("Now the first elem of t2 is {:?}", tier_vector_write_lock.x_stats[0]);

            }

        }
    });
    
    let native_options = NativeOptions{
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(my_app)}),
    );

    Ok(())
}


impl App for MyApp<>  {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
        egui::CentralPanel::default().show(ctx, |ui| { 
            ctx.set_visuals(Visuals::light());


            let mut tier_plot_lines = Vec::new();
            let colors = [egui::Color32::BLUE, egui::Color32::GREEN, egui::Color32::BLACK, egui::Color32::BROWN, egui::Color32::YELLOW];


            let raw_plot_line = Line::new(self.raw_data.read().unwrap().get_raw_data()).width(2.0).color(egui::Color32::RED);



            //to exclude final catch-all line use this self.tiers.iter().take(self.tiers.len() - 1).enumerate()
            for (i, tier) in self.tiers.iter().enumerate() {
                let color = colors[i];
                let line = Line::new(tier.read().unwrap().get_means())
                .width(2.0)
                .color(color);

            tier_plot_lines.push(line);
            }

            let plot = Plot::new("plot")
            .min_size(Vec2::new(800.0, 600.0));

            if ui.button("Halt Processing").clicked() {
                self.should_halt.store(true, Ordering::SeqCst);
            }

            plot.show(ui, |plot_ui| {
                 plot_ui.line(raw_plot_line);

                 for line in tier_plot_lines {
                    plot_ui.line(line);
                 }

                 
            //     plot_ui.line(initial_tier_plot_line);
            //     plot_ui.line(t2_plot_line);
            //     plot_ui.line(t3_plot_line);
            //     //plot_ui.line(t4_plot_line);
            });

        });
        ctx.request_repaint();
    }
}

/*
- MyApp<T> is a generic struct, T is a type that implements the DataStrategy trait. T can be either StdinData or ADWIN_window
- genercs in myapp<T> to allow it to be agnostic about the specific type of data it's working with, as long as that type conforms to the DataStrategy 

- integrate with eframe and egui, your MyApp struct needs to implement the App trait from the eframe crate.
- This trait requires you to define an update method, where you will handle drawing the UI and processing events.

- 'update' function creates the Egui window and where access MyApp data and methods, allowing to interact with the underlying data (handled by T) and reflect changes in the UI.
*/