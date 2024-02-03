mod adwin;

use std::io::{self, BufRead};
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
            adwin_plot: Arc::new(RwLock::new(ADWIN::new(0.0000000000000000000000000000000000000000000000000000000001))) ,
        }
    }
}
//delta
//1e-100
//0.0001

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let file_name = "variance_dataset.csv";
    //let file_name = "test.csv";

    let file = File::open(file_name)?;
    let rdr = io::BufReader::new(file);
    //let file_path = "variance_dataset_low_100.csv";

    let app = myApp::default();
    let adwin_plot_accesor = app.adwin_plot.clone();

    let reader_thread = thread::spawn(move || {
        for line in rdr.lines() {
            if let Ok(line) = line {
                //adwin_plot_accesor.lock().unwrap().append_str(line);
                adwin_plot_accesor.write().append_str(line);
                thread::sleep(Duration::from_millis(50));
            }
        }
    });

    let native_options = NativeOptions{
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(app)}),
    );

    reader_thread.join().unwrap();

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