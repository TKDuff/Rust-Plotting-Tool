mod adwin;
mod adwin_aggregate;

use adwin::ADWIN_window;
use adwin_aggregate::ADWIN_aggregate;


use eframe::epaint::mutex::RwLock;
use std::thread;
use std::sync::Arc;
use egui_plot :: {Line, Plot};
use eframe::{egui, NativeOptions};
use egui::{Vec2, Visuals};
use tokio::runtime::Runtime;
use tokio::io::{self, AsyncBufReadExt, BufReader}; 
use crossbeam::channel; 





struct myApp {
    adwin_window_plot: Arc<RwLock<ADWIN_window>>,
    adwin_aggregate_plot:  Arc<RwLock<ADWIN_aggregate>>,
}

impl Default for myApp {
    fn default() -> Self {
        Self {
            adwin_window_plot: Arc::new(RwLock::new(ADWIN_window::new(0.000000000000000000000000000001))) ,
            adwin_aggregate_plot: Arc::new(RwLock::new(ADWIN_aggregate::new())),
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

    let rt = Runtime::new().unwrap();
    let adwin_window_plot_accesor = app.adwin_window_plot.clone();

    rt.spawn(async move {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            tokio::select! {
                line = lines.next_line() => {
                    if let Ok(Some(line)) = line {
                        adwin_window_plot_accesor.write().append_str(line);
                        if let Some(cut_index) = adwin_window_plot_accesor.read().check_cut() {
                            rd_sender.send(cut_index).unwrap();
                        }
                    } else {
                        // End of input or an error. You can break or handle it as needed.
                        break;
                    }
                },
                point_means_result = rd_receiver.recv() => {
                    if let Some((x_mean, y_mean, len)) = point_means_result {
                        adwin_window_plot_accesor.write().cut_window(len, x_mean, y_mean);
                    }
                }, 
            }
        }
    });

    let aggregate_thread_window_plot_accessor = app.adwin_window_plot.clone();
    let aggregate_thread_aggregate_plot_accessor = app.adwin_aggregate_plot.clone();

    //Aggregate thread
    thread::spawn(move || {
        let mut chunk: Vec<[f64;2]>;


        for message in hd_receiver {
            if let cut_index = message {
                chunk =  aggregate_thread_window_plot_accessor.read().get_window_points()[..cut_index + 1].to_vec();
                //downsampler_raw_data_thread.write().append_chunk_aggregate_statistics(chunk);
                hd_sender.send(aggregate_thread_aggregate_plot_accessor.write().append_chunk_aggregate_statistics(chunk, cut_index));
            }
        }

    });


    let native_options = NativeOptions{
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(app)}),
    );


    Ok(())  
}


impl eframe::App for myApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| { 
            ctx.set_visuals(Visuals::light());

            let aggregate_plot_line = Line::new(self.adwin_aggregate_plot.read().get_aggregate_means()).width(2.0);
            let window_line = Line::new(self.adwin_window_plot.read().get_window_points()).width(2.0);

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