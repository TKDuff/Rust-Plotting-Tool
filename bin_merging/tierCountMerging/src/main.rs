
#![allow(warnings)] //Remove warning, be sure to remove this
use project_library::{AggregationStrategy, CountAggregateData, CountRawData, DataStrategy, TierData};
use tokio::task::spawn_blocking; //no need import 'bin.rs' Bin struct as is not used directly by main


use std::{num, thread};
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

    let args: Vec<String> = env::args().collect();
    let mut num_tiers = 0;

    let (rd_sender, hd_receiver) = channel::unbounded();
    let args: Vec<String> = env::args().collect();

    let data_strategy = args[1].as_str();
    let num_tiers = args[2].parse::<usize>().unwrap_or_default();

    println!("{} {}", data_strategy, num_tiers);  

    let my_app = match data_strategy {
        "count" => MyApp::new(
            Arc::new(RwLock::new(CountRawData::new())),
            create_tiers(num_tiers),
        Arc::new(AtomicBool::new(false)),
        ),
        // ... other cases ...
        _ => panic!("Invalid argument, please give an argument of one of the following\nadwin\ncount\ninterval"),
    };

    fn create_tiers(num_tiers: usize) -> Vec<Arc<RwLock<TierData>>> {
        (0..num_tiers).map(|_| Arc::new(RwLock::new(TierData::new()))).collect()
    }

    
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
    //let aggregate_thread_his_data_accessor = my_app.aggregate_data_tier.clone();

    
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
                //println!("{:?}", initial_tier_lock.print_x_means());
            }
            
        }   
    });

    
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();    
    // let t1_access = my_app.initial_tier.clone(); ####
    // let t2_access = my_app.tier2.clone();
    // let t3_access = my_app.tier3.clone();
    let tier_vector = my_app.tiers.clone();
    let tier_vector_length = num_tiers;     //this can be passed or known, don't need to check len
    
    let t = thread::spawn(move || { 
        // t3_access.write().unwrap().x_stats.drain(0..1); use 'tier_vector_lenght' to get last tier index, then drain
        // t3_access.write().unwrap().y_stats.drain(0..1);

        // let mut t1_length = 0;
        // let mut t2_length = 0;  
        // let mut t3_length = 0;

        // let mut merged_t3_last_x_element;
        // let mut merged_t3_last_y_element;

        loop {
            for tier in 0..2 {  //only testing on first tier, initial tier, for now
                // let current_tier = tier_vector[tier];
                // let lower_tier = tier_vector[tier+1];

                if tier_vector[tier].read().unwrap().x_stats.len() == 6 {
                    println!("\n{}", tier);
                    print!("{:?}", tier_vector[tier].read().unwrap().print_x_means("Before"));
                    process_tier(&tier_vector[tier], &tier_vector[tier+1], 7);
                    print!("{:?}\n", tier_vector[tier].read().unwrap().print_x_means("After"));
                }
            }

            /*
            t1_length = t1_access.read().unwrap().x_stats.len();
            if t1_length == 4 {
                process_tier(&t1_access, &t2_access, 7)
            }

            t2_length = t2_access.read().unwrap().x_stats.len();

            if t2_length == 4 {
                process_tier(&t2_access, &t3_access, 7)
            }

            t3_length = t3_access.read().unwrap().x_stats.len();

            if t3_length == 6 {
                merged_t3_last_x_element = t3_access.write().unwrap().merge_final_tier_vector_bins(3, true);
                merged_t3_last_y_element = t3_access.write().unwrap().merge_final_tier_vector_bins(3, false);
                println!("Got the point {:?}", merged_t3_last_x_element);
                println!("The first elem of t3 was {:?}", t2_access.read().unwrap().x_stats[0]);
                t2_access.write().unwrap().x_stats[0] = merged_t3_last_x_element;
                t2_access.write().unwrap().y_stats[0] = merged_t3_last_y_element;
                println!("Now the first elem of t3 is {:?}", t2_access.read().unwrap().x_stats[0]);

            }*/
        }
    });

    
    pub fn process_tier(current_tier: &Arc<RwLock<TierData>>, previous_tier: &Arc<RwLock<TierData>>, cut_length: usize) {
        let mut vec_len: usize;
        let current_tier_x_average;
        let current_tier_y_average;
        {
            let mut current_tier_lock = current_tier.write().unwrap();
            let vec_slice = current_tier_lock.get_slices(cut_length);
            current_tier_x_average = current_tier_lock.merge_vector_bins(vec_slice.0);
            current_tier_y_average = current_tier_lock.merge_vector_bins(vec_slice.1);
            vec_len = current_tier_lock.x_stats.len();

            current_tier_lock.x_stats[0] = current_tier_x_average;
            current_tier_lock.x_stats.drain(1..vec_len-1);

            current_tier_lock.y_stats[0] = current_tier_y_average;
            current_tier_lock.y_stats.drain(1..vec_len-1);

            //println!("{:?}", current_tier_lock.print_x_means());
        }

        {
            let mut previous_tier_lock = previous_tier.write().unwrap();
            previous_tier_lock.x_stats.push(current_tier_x_average);
            previous_tier_lock.y_stats.push(current_tier_y_average);  
        }

        //println!("\nTier 1 merge");
        //println!("{:?}", current_tier.write().unwrap().print_x_means());
        //println!("\n");

    }

    
    let native_options = NativeOptions{
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(my_app)}),
    );



    /*
    let (rd_sender, hd_receiver) = channel::unbounded();
    let (timer_sender, mut raw_data_receiver) = mpsc::unbounded_channel::<&str>();
    let (hd_sender, mut rd_receiver) = tokio::sync::mpsc::unbounded_channel::<(f64, f64, usize)>();


    let rt = Runtime::new().unwrap();

    let raw_data_thread = my_app.raw_data.clone();
    let require_external_trigger = raw_data_thread.read().unwrap().requires_external_trigger();

    rt.spawn(async move {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            tokio::select! {
                line = lines.next_line() => {
                    if let Ok(Some(line)) = line {
                        raw_data_thread.write().unwrap().append_str(line);
                        if !require_external_trigger {
                            if let Some(cut_index) = raw_data_thread.write().unwrap().check_cut() {
                                rd_sender.send(cut_index).unwrap();
                            }
                        }
                    } else {
                        // End of input or an error. You can break or handle it as needed.
                        break;
                    }
                }, 
                //Look into removing this if adwin or count strategies are selected, leaving in for now to make progress
                aggregate_signal = raw_data_receiver.recv(), if require_external_trigger => {
                    if let Some(signal) = aggregate_signal{
                        //re.sender must send data of same type (see cut_index above), if needs be can create ENUM to enscapsulate different type of message, for now its okay since interval message type not important
                        //rd_sender.send(0).unwrap();
                    }
                },
                /*May not need this anymore...
                - With tiered approach initial aggregation tier keeps reference to first element of raw data vector
                point_means_result = rd_receiver.recv() => {
                    if let Some((x_mean, y_mean, len)) = point_means_result {
                        raw_data_thread.write().unwrap().remove_chunk(len, (x_mean, y_mean));
                    }
                },*/
            }
        }
    });

    /*Asynchronous timer
    - Nessecary as the tokio loop reads from standard input, which is a blocking task
    - Upon checking, may not be necessary, thus don't need 'aggregate_signal'*/
    rt.spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;
            timer_sender.send("should put something here");
        }
    });


    let aggregate_thread_raw_data_accessor = my_app.raw_data.clone();
    let aggregate_thread_aggregate_data_accessor = my_app.aggregate_data_tier.clone();

    
    //Aggregate points thread
    thread::spawn(move || {
        let mut chunk: Vec<[f64;2]>;
        let mut objective_length = 0;
        
        for chunk in hd_receiver {
            //chunk = aggregate_thread_raw_data_accessor.read().unwrap().get_chunk(message);
            println!("Processing the chunk: {:?}\n", chunk);
            aggregate_thread_aggregate_data_accessor.write().unwrap().append_chunk_aggregate_statistics(chunk);
            //hd_sender.send(aggregate_thread_aggregate_data_accessor.write().unwrap().append_chunk_aggregate_statistics(chunk));
        }

    });

    /*
    let native_options = NativeOptions{
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(my_app)}),
    );*/

    Ok(())*/
    Ok(())
}


impl App for MyApp<>  {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
        egui::CentralPanel::default().show(ctx, |ui| { 
            ctx.set_visuals(Visuals::light());


            let mut tier_plot_lines = Vec::new();
            let colors = [egui::Color32::BLUE, egui::Color32::GREEN, egui::Color32::BLACK, egui::Color32::BROWN];


            let raw_plot_line = Line::new(self.raw_data.read().unwrap().get_raw_data()).width(2.0).color(egui::Color32::RED);
            // let initial_tier_plot_line = Line::new(self.initial_tier.read().unwrap().get_means()).width(2.0).color(egui::Color32::BLUE);
            // let t2_plot_line = Line::new(self.tier2.read().unwrap().get_means()).width(2.0).color(egui::Color32::GREEN);
            // let t3_plot_line = Line::new(self.tier3.read().unwrap().get_means()).width(2.0).color(egui::Color32::BLACK);
            //let t4_plot_line = Line::new(self.tier4.read().unwrap().get_means()).width(2.0).color(egui::Color32::BROWN);


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