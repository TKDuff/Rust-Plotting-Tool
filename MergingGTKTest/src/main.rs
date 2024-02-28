
#![allow(warnings)] use nix::libc::ARPD_FLUSH;
//Remove warning, be sure to remove this
use project_library::{CountRawData, DataStrategy, TierData, Bin, main_threads, process_tier, setup_my_app};
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
use gtk4::prelude::*; 
use gtk4::{Application, ApplicationWindow, Label}; 


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
    //let (rd_sender, hd_receiver) = channel::unbounded();

    let (aggregation_strategy, strategy, tiers, catch_all_policy, should_halt, num_tiers)  =  setup_my_app()?;
     

    let my_app = MyApp::new(aggregation_strategy, tiers, should_halt);
    let should_halt_clone = my_app.should_halt.clone();


    let raw_data_thread_for_setup = my_app.raw_data.clone();
    
    let raw_data_accessor = my_app.raw_data.clone();
    let initial_tier_accessor = my_app.tiers[0].clone();
    let tier_vector = my_app.tiers.clone();


    /*
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
    */
    //let rt = Runtime::new().unwrap();
    let raw_data_thread = my_app.raw_data.clone();
    
    //create tokio thread to read in standard input, then append to live data vector
    // if strategy == "count" {
    //     main_threads::create_count_stdin_read(&rt, should_halt_clone, raw_data_thread, rd_sender);
    // } else {
    //     main_threads::create_interval_stdin_read(&rt, should_halt_clone, raw_data_thread, rd_sender);
    // }
    //create threads in reverse order of who access live data. Tokio last, as want other merging threads to be ready to handle aggregate live data
    
    let app = Application::new(Some("com.example.plot"), Default::default());    

    app.connect_activate(|app| {

    let window = ApplicationWindow::builder()
            .application(app)
            .title("Plot test")
            .default_width(300)
            .default_height(100)
            .build();

            let label = Label::new(None);
            window.set_child(Some(&label));
            window.present();
    });


    app.run();

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

    Ok(())



    

}


