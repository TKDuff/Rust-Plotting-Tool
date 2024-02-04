mod adwin;

//use std::io::{self, BufRead};
use std::os::fd::AsRawFd;
use eframe::epaint::mutex::RwLock;
use serde::Deserialize;
use std::io::{Read, Write};
use adwin::ADWIN;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::sync::{Arc, Mutex};
use egui_plot :: {Line, Plot};
use eframe::{egui, NativeOptions};
use egui::{Vec2, CentralPanel, style, Visuals};
use tokio::runtime::Runtime;
use tokio::io::{self, AsyncBufReadExt, BufReader}; 
use crossbeam::channel; 


#[derive(Debug, Deserialize)]
struct Record {
    x_col: f64,
    y_col: f64,
}

struct myApp {
    adwin_plot: Arc<RwLock<ADWIN>>,
}

impl Default for myApp {
    fn default() -> Self {
        Self {
            adwin_plot: Arc::new(RwLock::new(ADWIN::new(0.000000000000000000000000000001))) ,
        }
    }
}
//delta
//1e-100
//0.0001

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rd_sender, hd_receiver) = channel::unbounded();
    let (hd_sender, mut rd_receiver) = tokio::sync::mpsc::unbounded_channel::<(f64, f64, usize)>();
    let app = myApp::default();
    //let file_name = "person.csv";
    let file_name = "variance_dataset.csv";

    /*
    let file = File::open(file_name)?;
    let rdr = io::BufReader::new(file);
    //let file_path = "variance_dataset_low_100.csv";

    let adwin_plot_accesor = app.adwin_plot.clone();
    let reader_thread = thread::spawn(move || {
        for line in rdr.lines() {
            if let Ok(line) = line {
                //adwin_plot_accesor.lock().unwrap().append_str(line);
                adwin_plot_accesor.write().append_str(line);
                thread::sleep(Duration::from_millis(100));
            }
        }
    });*/


    let rt = Runtime::new().unwrap();
    let adwin_plot_accesor = app.adwin_plot.clone();

    rt.spawn(async move {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            tokio::select! {
                line = lines.next_line() => {
                    if let Ok(Some(line)) = line {
                        adwin_plot_accesor.write().append_str(line);
                        if let (Some(cut_index), mean_y) = adwin_plot_accesor.read().check_cut() {;
                            rd_sender.send((cut_index, mean_y)).unwrap();
                        }
                    } else {
                        // End of input or an error. You can break or handle it as needed.
                        break;
                    }
                },
                point_means_result = rd_receiver.recv() => {
                    if let Some((x_mean, y_mean, len)) = point_means_result {
                        adwin_plot_accesor.write().cut_window(len, x_mean, y_mean);
                    }
                }, 
            }
        }
    });

    let downsampler_raw_data_thread = app.adwin_plot.clone();

    let historic_data_handle = thread::spawn(move || {
        let mut chunk: Vec<[f64;2]>;
        let mut objective_length = 0;

        for message in hd_receiver {
            if let (cut_index, _mean_y) = message {
                chunk =  downsampler_raw_data_thread.read().get_window_points()[..cut_index + 1].to_vec();
                //downsampler_raw_data_thread.write().append_chunk_aggregate_statistics(chunk);
                hd_sender.send(downsampler_raw_data_thread.write().append_chunk_aggregate_statistics(chunk, cut_index));
            }
        }

    });


    let native_options = NativeOptions{
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(app)}),
    );

    //reader_thread.join().unwrap();

    Ok(())  
}


impl eframe::App for myApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| { 
            ctx.set_visuals(Visuals::light());

            let aggregate_plot_line = Line::new(self.adwin_plot.read().get_aggregate_points()).width(2.0);
            let window_line = Line::new(self.adwin_plot.read().get_window_points()).width(2.0);

            let plot = Plot::new("plot")
            .min_size(Vec2::new(400.0, 300.0));

            plot.show(ui, |plot_ui| {
                plot_ui.line(aggregate_plot_line);
                plot_ui.line(window_line);
            });

        });
        
        ctx.request_repaint();
    }
}

//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/RustTesting/aggregating_adaptive_window/ && cargo run --bin aggregating_adaptive_window)