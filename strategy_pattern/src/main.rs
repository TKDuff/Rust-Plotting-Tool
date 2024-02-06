
#![allow(warnings)] //Remove warning, be sure to remove this
mod data_strategy;
use data_strategy::DataStrategy;

mod interval_data;
use interval_data::IntervalRawData;

mod adwin_data;
use adwin_data::AdwinRawData;

mod count_data;
use count_data::CountRawData;

mod aggregation_strategy;
use aggregation_strategy::AggregationStrategy;


mod interval_aggregation;
use interval_aggregation::IntervalAggregateData;

mod adwin_aggregation;
use adwin_aggregation::AdwinAggregateData;

mod count_aggregation;
use count_aggregation::CountAggregateData;


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
        Some("adwin") => MyApp::new(
            Arc::new(RwLock::new(AdwinRawData::new())),
        Arc::new(RwLock::new(AdwinAggregateData::new())),
        ),
        Some("count") => MyApp::new(
            Arc::new(RwLock::new(CountRawData::new())),
        Arc::new(RwLock::new(CountAggregateData::new())),
        ),
        Some("interval") => MyApp::new(
            Arc::new(RwLock::new(IntervalRawData::new())),
        Arc::new(RwLock::new(IntervalAggregateData::new())),
        ),
        // ... other cases ...
        _ => panic!("Invalid argument"),
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

    /*Asynchronous timer */
    rt.spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;
            timer_sender.send("test");
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

//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/strategy_pattern/ && cargo run --bin strategy_pattern)

/*
- MyApp<T> is a generic struct, T is a type that implements the DataStrategy trait. T can be either StdinData or ADWIN_window
- genercs in myapp<T> to allow it to be agnostic about the specific type of data it's working with, as long as that type conforms to the DataStrategy 

- integrate with eframe and egui, your MyApp struct needs to implement the App trait from the eframe crate.
- This trait requires you to define an update method, where you will handle drawing the UI and processing events.

- 'update' function creates the Egui window and where access MyApp data and methods, allowing to interact with the underlying data (handled by T) and reflect changes in the UI.
*/