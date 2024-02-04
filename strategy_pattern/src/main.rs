
#![allow(warnings)] //Remove warning, be sure to remove this
mod data_strategy;
use data_strategy::DataStrategy;

mod interval_data;
use interval_data::StdinData;




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

struct MyApp<T: DataStrategy + Send + Sync> {
    raw_data: Arc<RwLock<T>>,
    // other fields...
}

impl<T: DataStrategy + Send + Sync> MyApp<T> {
    pub fn new(raw_data: T) -> Self {
        Self {
            raw_data: Arc::new(RwLock::new(raw_data)),
            // Initialize other fields...
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_app = MyApp::new(StdinData::new());

    let raw_data_thread = my_app.raw_data.clone();

    raw_data_thread.write().unwrap().append_str(String::from("123 45.67"));
    raw_data_thread.write().unwrap().append_str(String::from("246 91"));

    println!("{:?}", raw_data_thread.read().unwrap().get_raw_data());


    let native_options = NativeOptions{
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(my_app)}),
    );

    Ok(())
}


impl<T: DataStrategy + Send + Sync> App for MyApp<T>  {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
        egui::CentralPanel::default().show(ctx, |ui| { 
            ctx.set_visuals(Visuals::light());

            let raw_plot_line = Line::new(self.raw_data.read().unwrap().get_raw_data()).width(2.0);

            let plot = Plot::new("plot")
            .min_size(Vec2::new(800.0, 600.0));

            plot.show(ui, |plot_ui| {
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