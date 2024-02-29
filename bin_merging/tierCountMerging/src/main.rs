
#![allow(warnings)] use nix::libc::ARPD_FLUSH;
//Remove warning, be sure to remove this
use project_library::{CountRawData, DataStrategy, TierData, Bin, main_threads, process_tier, setup_my_app};
use std::fmt::format;
use std::process::id;
use std::{num, thread, usize};
use eframe::{egui, NativeOptions, App}; 
use egui::{Style, Visuals, ViewportBuilder};
use egui_plot :: {BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner, Legend, Line, Plot, PlotPoint, PlotResponse};
use egui::{Vec2, CentralPanel, Id};
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
    clicked_bin:  Option<(Bin, Bin)>,
}

impl MyApp {

    pub fn new(
        raw_data: Arc<RwLock<dyn DataStrategy + Send + Sync>>, 
        tiers: Vec<Arc<RwLock<TierData>>>,
        should_halt: Arc<AtomicBool>,
        clicked_bin:  Option<(Bin, Bin)>,
        
    ) -> Self {
        Self { raw_data, tiers ,should_halt, clicked_bin }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rd_sender, hd_receiver) = channel::unbounded();

    let (aggregation_strategy, strategy, tiers, catch_all_policy, should_halt, num_tiers)  =  setup_my_app()?;
     

    let my_app = MyApp::new(aggregation_strategy, tiers, should_halt, None);
    
    let should_halt_clone = my_app.should_halt.clone();


    let raw_data_thread_for_setup = my_app.raw_data.clone();
    
    let raw_data_accessor = my_app.raw_data.clone();
    let initial_tier_accessor = my_app.tiers[0].clone();
    let tier_vector = my_app.tiers.clone();

    //create the thread to handle tier merging
    if strategy == "count" {
        setup_count(raw_data_accessor, initial_tier_accessor, num_tiers, catch_all_policy, tier_vector);
    } else {
        setup_interval(raw_data_accessor, initial_tier_accessor, num_tiers, catch_all_policy, tier_vector)
    }

    let raw_data_accessor_for_thread = my_app.raw_data.clone();
    let initial_tier_accessor_for_thread = my_app.tiers[0].clone();
    //create thread to handle the live data being pushed to the initial tier
    main_threads::create_raw_data_to_initial_tier(hd_receiver, raw_data_accessor_for_thread, initial_tier_accessor_for_thread);

    let rt = Runtime::new().unwrap();
    let raw_data_thread = my_app.raw_data.clone();
    //create tokio thread to read in standard input, then append to live data vector
    if strategy == "count" {
        main_threads::create_count_stdin_read(&rt, should_halt_clone, raw_data_thread, rd_sender);
    } else {
        main_threads::create_interval_stdin_read(&rt, should_halt_clone, raw_data_thread, rd_sender);
    }
    //create threads in reverse order of who access live data. Tokio last, as want other merging threads to be ready to handle aggregate live data



    fn setup_count(raw_data_accessor: Arc<RwLock<dyn DataStrategy + Send + Sync>>, initial_tier_accessor: Arc<RwLock<TierData>>, num_tiers: usize, catch_all_policy: bool, tier_vector: Vec<Arc<RwLock<TierData>>>) {
        if num_tiers == 4 {
            main_threads::count_rd_to_ca_edge(raw_data_accessor, initial_tier_accessor); 
        } else {
            let num_tiers = tier_vector.len();
            let catch_all_tier = tier_vector[num_tiers-1].clone(); //correctly gets the catch all tier, have to minus one since len not 0 indexed 
    
            catch_all_tier.write().unwrap().x_stats.drain(0..1);
            catch_all_tier.write().unwrap().y_stats.drain(0..1);
    
            if catch_all_policy {
                main_threads::count_check_cut_ca(tier_vector, catch_all_tier, num_tiers);
            } else {
                main_threads::count_check_cut_no_ca(tier_vector, catch_all_tier, num_tiers);
            }
        }
    }

    fn setup_interval(raw_data_accessor: Arc<RwLock<dyn DataStrategy + Send + Sync>>, initial_tier_accessor: Arc<RwLock<TierData>>, num_tiers: usize, catch_all_policy: bool, tier_vector: Vec<Arc<RwLock<TierData>>>) {
        if num_tiers == 4 {
            main_threads::rd_to_ca_edge(raw_data_accessor, initial_tier_accessor);
        } else {
            let num_tiers = tier_vector.len();
            let catch_all_tier = tier_vector[num_tiers-1].clone(); //correctly gets the catch all tier, have to minus one since len not 0 indexed 

            catch_all_tier.write().unwrap().x_stats.drain(0..1);
            catch_all_tier.write().unwrap().y_stats.drain(0..1);

            if catch_all_policy {
                main_threads::interval_check_cut_ca(tier_vector, catch_all_tier, num_tiers)
            } else {        
                main_threads::interval_check_cut_no_ca(tier_vector, catch_all_tier, num_tiers);
            }
        }
    }
    

    let native_options = NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([1900.0, 800.0]),
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
            let mut tier_plot_lines_length: Vec<usize> = Vec::new();
            let mut number_of_tiers = self.tiers.len();
            let colors = [egui::Color32::BLUE, egui::Color32::GREEN, egui::Color32::BLACK, egui::Color32::BROWN, egui::Color32::YELLOW];

            
            let mut position: Option<PlotPoint> =None;
            let mut hovered_item: Option<String> = None;


            let plot_width = 1800.0;
            let plot_height = 600.0;
            let plot_top_left = egui::pos2(10.0, 50.0); 
            let bin_info_area_pos = egui::pos2(plot_top_left.x, plot_top_left.y + plot_height);

            let raw_plot_line = Line::new(self.raw_data.read().unwrap().get_raw_data()).width(2.0).color(egui::Color32::RED);
            //to exclude final catch-all line use this self.tiers.iter().take(self.tiers.len() - 1).enumerate()
            for (i, tier) in self.tiers.iter().enumerate() {
                let color = colors[i];
                let line_id = format!("Tier {}", i+1);
                let values = tier.read().unwrap().get_means(); 
                tier_plot_lines_length.push(values.len());  //want to store length of each line
                let line = Line::new(values)
                .width(2.0)
                .color(color)
                .name(&line_id)
                .id(egui::Id::new(i));

            tier_plot_lines.push(line);
            }

            let mut plot = Plot::new("plot").width(plot_width).height(plot_height).legend(Legend::default()).coordinates_formatter(Corner::LeftBottom, CoordinatesFormatter::default());

            
            if ui.button("Halt Processing").clicked() {
                self.should_halt.store(true, Ordering::SeqCst);
            }

            let plot_responese: PlotResponse<()> = plot.show(ui, |plot_ui| {
                 plot_ui.line(raw_plot_line);
                 for line in tier_plot_lines {
                    plot_ui.line(line);
                 }
                 position = plot_ui.pointer_coordinate();
            });

            let click = ctx.input(|i| i.pointer.any_click());

            if click {
                let tier_index = plot_responese.hovered_plot_item
                .and_then(|id| (0..self.tiers.len()).find(|&i| id == egui::Id::new(i)))
                .unwrap_or_else(|| usize::MAX);
            
                if tier_index != usize::MAX {
                self.clicked_bin = find_closest(position, &self.tiers[tier_index])
                }
            }

            egui::Area::new("Bin Information Area")
                .fixed_pos(bin_info_area_pos) // Position the area as desired
                .show(ui.ctx(), |ui| {
                    for i in 0..number_of_tiers {
                        ui.label(format!("Tier {} length {}", i+1, tier_plot_lines_length[i]));
                    }

                    if let Some((x_bin, y_bin)) = self.clicked_bin {
                        // Only display the information if clicked_info is Some
                        ui.label(format!("X: Mean = {:.2}", x_bin.mean));
                        ui.label(format!("X: Sum = {:.2}", x_bin.sum));
                        ui.label(format!("X: Min = {:.2}", x_bin.min));
                        ui.label(format!("X: Max = {:.2}", x_bin.max));
                        ui.label(format!("X: Count = {:.2}", x_bin.count));
                    } else {
                        // Display some default text or leave it empty
                        ui.label("Click on a plot point");
                    }
                });


        });
        ctx.request_repaint();
    }
}


fn find_closest(position: Option<PlotPoint>, tier: &Arc<RwLock<TierData>>) -> Option<(Bin, Bin)>  {
    let plot_point = position?;// ? returns None is position is None, no need to check via 'Some' statement

    let x = plot_point.x;
    let y = plot_point.y;
    let tier_data = tier.read().unwrap();

    let tolerance = 5.0;
    tier_data.x_stats.iter().zip(tier_data.y_stats.iter())  //combine x_stats and y_stats iterator via zip
        .find(|&(x_bin, y_bin)| {   //find takes x_bin and y_bin which is each element, used to get absolute difference of means
            (x - x_bin.get_mean()).abs() <= tolerance && (y - y_bin.get_mean()).abs() <= tolerance //get absolute diff of means and check they are within tolerance 
        })
        .map(|(x_closest, y_closest)| (*x_closest, *y_closest)) //map extracts pair from the find, return them as a tuple
} 

/*
- MyApp<T> is a generic struct, T is a type that implements the DataStrategy trait. T can be either StdinData or ADWIN_window
- genercs in myapp<T> to allow it to be agnostic about the specific type of data it's working with, as long as that type conforms to the DataStrategy 

- integrate with eframe and egui, your MyApp struct needs to implement the App trait from the eframe crate.
- This trait requires you to define an update method, where you will handle drawing the UI and processing events.

- 'update' function creates the Egui window and where access MyApp data and methods, allowing to interact with the underlying data (handled by T) and reflect changes in the UI.
*/