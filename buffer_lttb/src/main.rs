mod data_module;
use data_module::{StdinData, DownsampledData};



use std::thread;
use eframe::{egui, NativeOptions}; 
use egui_plot :: {BoxElem, BoxPlot, BoxSpread, Legend, Line, Plot};
use egui::{Vec2, CentralPanel};
use std::io::{self, BufRead};
use std::sync::{Arc, RwLock};
use crossbeam::channel;
use std::time::Duration;

struct MyApp {
    raw_data: Arc<RwLock<StdinData>>,
    historic_data: Arc<RwLock<DownsampledData>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            raw_data: Arc::new(RwLock::new(StdinData::new())),
            historic_data: Arc::new(RwLock::new(DownsampledData::new())),
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let (rd_sender, hd_receiver) = channel::unbounded();
    let (hd_sender, rd_receiver) = channel::unbounded();

    let my_app = MyApp::default();

    //let sender_clone = sender.clone();
    let raw_data_thread = my_app.raw_data.clone();

    let raw_data_handle = thread::spawn(move || { 
        let stdin = io::stdin();          //global stdin instance
        let locked_stdin = stdin.lock();  //lock stdin for exclusive access
        let mut length = 0;
        let mut points_count = 51;

        for line in locked_stdin.lines() {
            let line_string = line.unwrap();
            raw_data_thread.write().unwrap().append_str(&line_string);
            length = raw_data_thread.read().unwrap().get_length();
            if length % points_count == 0 {
                rd_sender.send((length, points_count)).unwrap();
            }
            if let Ok(point_means) = rd_receiver.try_recv() {
                raw_data_thread.write().unwrap().remove_chunk(points_count, point_means);
            }
        }
    });

    let downsampler_raw_data_thread = my_app.raw_data.clone();
    let downsampler_thread = my_app.historic_data.clone();

    
    let historic_data_handle = thread::spawn(move || {
        let mut chunk: Vec<[f64;2]>;
        let mut objective_length = 0;
        let lltb_points = 25;
        
        for message in hd_receiver {
            let(raw_data_length, point_count) = message;
            chunk = downsampler_raw_data_thread.read().unwrap().get_chunk(point_count);
            hd_sender.send(downsampler_thread.write().unwrap().append_statistics(chunk, point_count));
            objective_length += 1;
            if objective_length % lltb_points == 0 {
                downsampler_thread.write().unwrap().combineBins(lltb_points);
            }
        }
    });

    
    let native_options = NativeOptions{
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(my_app)}),
    );

    raw_data_handle.join().unwrap();
    historic_data_handle.join().unwrap();

    Ok(())
}

impl eframe::App for MyApp {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
        egui::CentralPanel::default().show(ctx, |ui| { 
          
            //let raw_data_points = self.raw_data.read().unwrap().get_values();
            //let historic_data_points = self.raw_data.read().unwrap().get_values();

            let raw_plot_line = Line::new(self.raw_data.read().unwrap().get_values());
            let historic_plot_line = Line::new(self.historic_data.read().unwrap().get_means());

            let plot = Plot::new("plot")
            .min_size(Vec2::new(400.0, 300.0));

            plot.show(ui, |plot_ui| {
                plot_ui.line(historic_plot_line);
                plot_ui.line(raw_plot_line);
            });
        });
        ctx.request_repaint();
    }
}
