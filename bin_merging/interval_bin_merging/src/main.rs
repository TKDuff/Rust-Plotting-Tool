#![allow(warnings)] //Remove warning, be sure to remove this
#![allow(dead_code, unused_variables)]
use project_library::{CountAggregateData, CountRawData, AggregationStrategy, DataStrategy}; //no need import 'bin.rs' Bin struct as is not used directly by main



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


struct MyApp {
    raw_data: Arc<RwLock<dyn DataStrategy + Send + Sync>>,  //'dyn' mean 'dynamic dispatch', specified for that instance. Allow polymorphism for that instance, don't need to know concrete type at compile time
    aggregate_data: Arc<RwLock<dyn AggregationStrategy + Send + Sync>>,
}

impl MyApp {
    pub fn new(
        raw_data: Arc<RwLock<dyn DataStrategy + Send + Sync>>, 
        aggregate_data: Arc<RwLock<dyn AggregationStrategy + Send + Sync>>
    ) -> Self {
        Self { raw_data, aggregate_data }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let my_app = match args.get(1).map(String::as_str) {
        Some("count") => MyApp::new(
            Arc::new(RwLock::new(CountRawData::new())),
        Arc::new(RwLock::new(CountAggregateData::new())),
        ),
        // ... other cases ...
        _ => panic!("Invalid argument, please give an argument of one of the following\nadwin\ncount\ninterval"),
    };

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
                        rd_sender.send(0).unwrap();
                    }
                },
                point_means_result = rd_receiver.recv() => {
                    if let Some((x_mean, y_mean, len)) = point_means_result {
                        raw_data_thread.write().unwrap().remove_chunk(len, (x_mean, y_mean));
                    }
                },
            }
        } 
    }); 

    let t1_async_interval_task_aggregate_accessor = my_app.aggregate_data.clone();

    /*Asynchronous timers
    For now using tokio async taks, will move to using full threads as vector merging is CPU bound, thus need threads. Async speed up development
    First tier always decrement returned value by 1 for first tier of async task since wan't to keep first element to maintain plot consistency, from aggregate plot to raw data plot

    Will have interval thread creation determined by user, for now hardcoded to get working. 
    */
    /*1st tier */
    rt.spawn(async move {
        let mut seconds_length = 0;
        let interval_duration = 1;
        let interval_duration_millis = interval_duration*1000;
        let mut interval = time::interval(Duration::from_secs(interval_duration));

        loop {
            interval.tick().await;

            //decrement returned value by 1 for first tier of async task since wan't to keep first element to maintain plot consistency, from aggregate plot to raw data plot
            seconds_length = (t1_async_interval_task_aggregate_accessor.write().unwrap().categorise_recent_bins(interval_duration_millis as u128, seconds_length)) - 1; 
            println!("\n1 second tick");   
        }
    });

    let t2_async_interval_task_aggregate_accessor = my_app.aggregate_data.clone();
    /*2nd tier */
    rt.spawn(async move {
        let mut minute_length = 0;
        let interval_duration = 10;
        let interval_duration_millis = interval_duration*1000;
        let mut interval = time::interval(Duration::from_secs(interval_duration));

        loop {
            interval.tick().await;
            minute_length = t2_async_interval_task_aggregate_accessor.write().unwrap().categorise_recent_bins_t2(interval_duration_millis as u128, minute_length);
        }
    });


    let aggregate_thread_raw_data_accessor = my_app.raw_data.clone();
    let aggregate_thread_aggregate_data_accessor = my_app.aggregate_data.clone();

    
    //Aggregate points thread
    thread::spawn(move || {
        let mut chunk: Vec<[f64;2]>;
        let mut objective_length = 0;

        
        for message in hd_receiver {
            chunk = aggregate_thread_raw_data_accessor.read().unwrap().get_chunk(message);
            hd_sender.send(aggregate_thread_aggregate_data_accessor.write().unwrap().append_chunk_aggregate_statistics(chunk));
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

            let raw_plot_line = Line::new(self.raw_data.read().unwrap().get_raw_data()).width(2.0);
            let historic_plot_line = Line::new(self.aggregate_data.read().unwrap().get_means()).width(2.0);

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