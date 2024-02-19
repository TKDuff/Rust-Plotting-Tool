
#![allow(warnings)] //Remove warning, be sure to remove this
use project_library::{AggregationStrategy, CountAggregateData, CountRawData, DataStrategy, TierData};
use tokio::task::spawn_blocking; //no need import 'bin.rs' Bin struct as is not used directly by main


use std::thread;
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

struct MyApp {
    raw_data: Arc<RwLock<dyn DataStrategy + Send + Sync>>,  //'dyn' mean 'dynamic dispatch', specified for that instance. Allow polymorphism for that instance, don't need to know concrete type at compile time
    aggregate_data_tier: Arc<RwLock<dyn AggregationStrategy + Send + Sync>>,
    tier2: Arc<RwLock<TierData>>
}

impl MyApp {

    pub fn new(
        raw_data: Arc<RwLock<dyn DataStrategy + Send + Sync>>, 
        aggregate_data_tier: Arc<RwLock<dyn AggregationStrategy + Send + Sync>>,
        tier2: Arc<RwLock<TierData>>
    ) -> Self {
        Self { raw_data, aggregate_data_tier, tier2 }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rd_sender, hd_receiver) = channel::unbounded();
    let args: Vec<String> = env::args().collect();

    let my_app = match args.get(1).map(String::as_str) {
        Some("count") => MyApp::new(
            Arc::new(RwLock::new(CountRawData::new())),
        Arc::new(RwLock::new(CountAggregateData::new())),
        Arc::new(RwLock::new(TierData::new())),
        ),
        // ... other cases ...
        _ => panic!("Invalid argument, please give an argument of one of the following\nadwin\ncount\ninterval"),
    };

    
    let rt = Runtime::new().unwrap();
    let raw_data_thread = my_app.raw_data.clone();

    rt.spawn(async move {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();
        
        loop {
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
    let aggregate_thread_his_data_accessor = my_app.aggregate_data_tier.clone();


    thread::spawn(move || {
        let mut chunk: Vec<[f64;2]>;
        let mut objective_length = 0;
        let average = 0;        
        for message in hd_receiver {
            chunk = aggregate_thread_raw_data_accessor.read().unwrap().get_chunk(message);
            aggregate_thread_his_data_accessor.write().unwrap().append_chunk_aggregate_statistics(chunk);
            aggregate_thread_raw_data_accessor.write().unwrap().remove_chunk(7);
        }   
    });

    
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();    
    let agg_data_access = my_app.aggregate_data_tier.clone();

    
    let t = thread::spawn(move || { 
        let mut agg_data_length = 0;    
        loop {
            agg_data_length = agg_data_access.read().unwrap().get_length();
            //println!("test {}", agg_data_length);
            if agg_data_length == 5 {
                //x_bin = agg_data_access.write().unwrap().merge_vector_bins_x();
                //y_bin = agg_data_access.write().unwrap().merge_vector_bins_y();
            }
        }
    });

    /*
    pub fn process_tier(current_tier:  Arc<RwLock<dyn AggregationStrategy + Send + Sync>> , lower_tier: Arc<RwLock<dyn AggregationStrategy + Send + Sync>>, cut_length: usize) {
        let mut vec_len: usize;
        let current_tier_x_average;
        let current_tier_y_average;

        {
            let mut current_tier_lock = current_tier.write().unwrap();
            let vec_slice = current_tier_lock.get_slices(cut_length);
            current_tier_x_average = current_tier_lock.merge_vector_bins(vec_slice.0);
            current_tier_y_average = current_tier_lock.merge_vector_bins(vec_slice.1);
            vec_len = current_tier_lock.get_length();

            current_tier_lock.misc_x(current_tier_x_average, vec_len);
            current_tier_lock.misc_y(current_tier_y_average, vec_len);
        }
    }*/

    

    pub fn priority_merge_dispatcher(elapsed: u64, chunk: Option<Vec<[f64;2]>>, raw_data: Arc<RwLock<dyn DataStrategy + Send + Sync>>,  tier: Arc<RwLock<dyn AggregationStrategy + Send + Sync>>) {

        println!("{:?}", chunk);

        if let Some(actual_chunk) = chunk {
            println!("test");
            /*
            rayon::spawn(move || {

            let rd_average = tier.write().unwrap().append_chunk_aggregate_statistics(actual_chunk);
            //let last_elment = raw_data.write().unwrap().remove_chunk(5);

            });*/
        }

        /*
        if elapsed % 1 == 0 {
            println!("Merging for Tier 1 {}", elapsed);
        }

         if elapsed % 4 == 0 {
            println!("Merging for Tier 2 {}", elapsed);
        }*/
    }

    fn example(chunk: Vec<[f64;2]>) {
        // Process the chunk data
        println!("Processing chunk in thread pool: {:?}", chunk);
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

            let raw_plot_line = Line::new(self.raw_data.read().unwrap().get_raw_data()).width(2.0);
            let historic_plot_line = Line::new(self.aggregate_data_tier.read().unwrap().get_means()).width(2.0);

            let plot = Plot::new("plot")
            .min_size(Vec2::new(800.0, 600.0));

            plot.show(ui, |plot_ui| {
                plot_ui.line(historic_plot_line);
                plot_ui.line(raw_plot_line);
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