#![allow(warnings)] //Remove warning, be sure to remove this
mod data_module;
use data_module::{StdinData, DownsampledData};
use tokio::runtime::Runtime;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::time::{self, Duration, Interval};
use tokio::fs::File;
use std::thread;
use eframe::{egui, NativeOptions}; 
use egui::{Style, Visuals};
use egui_plot :: {BoxElem, BoxPlot, BoxSpread, Legend, Line, Plot};
use egui::{Vec2, CentralPanel};
use std::sync::{Arc, RwLock};
use crossbeam::channel;

struct MyApp {
    raw_data: Arc<RwLock<StdinData>>,
    historic_data: Arc<RwLock<DownsampledData>>,
}

impl Default for MyApp {
    fn default() ->  Self {
        Self {
            raw_data: Arc::new(RwLock::new(StdinData::new())),
            historic_data: Arc::new(RwLock::new(DownsampledData::new())),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rd_sender, hd_receiver) = channel::unbounded();
    let rt = Runtime::new().unwrap();

    let my_app = MyApp::default();
    let raw_data_thread = my_app.raw_data.clone();

    rt.spawn(async move {

        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        let mut interval = time::interval(Duration::from_secs(1));
        let mut line_count = 0;
        let  points_count = 10;
        let mut length = 0;

        loop {
            tokio::select! {
                line = lines.next_line() => {
                    if let Ok(Some(line)) = line {
                        println!("{}", line);
                        raw_data_thread.write().unwrap().append_str(line);
                        line_count += 1;
                    } else {
                        // End of input or an error. You can break or handle it as needed.
                        break;
                    }
                },
                _ = interval.tick() => {
                    rd_sender.send(line_count).unwrap();
                }
            }

        }
    });

    let downsampler_raw_data_thread = my_app.raw_data.clone();
    let historic_data_handle = thread::spawn(move || {
        let mut chunk: Vec<[f64;2]>;
        let mut objective_length = 0;

        for message in hd_receiver {
            println!("{:?}", downsampler_raw_data_thread.read().unwrap().get_chunk(message));
            downsampler_raw_data_thread.write().unwrap().remove_chunk(message);
        }

    });

    historic_data_handle.join().unwrap();

    Ok(())
}

impl eframe::App for MyApp {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
        egui::CentralPanel::default().show(ctx, |ui| { 
            ctx.set_visuals(Visuals::light());
            //let raw_data_points = self.raw_data.read().unwrap().get_values();
            //let historic_data_points = self.raw_data.read().unwrap().get_values();

            let raw_plot_line = Line::new(self.raw_data.read().unwrap().get_values()).width(2.0);
            let historic_plot_line = Line::new(self.historic_data.read().unwrap().get_means()).width(2.0);

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


//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/combiningBins/ && cargo run --bin combiningBins)