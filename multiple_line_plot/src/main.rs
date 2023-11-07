mod plot_one;

use eframe::{egui, NativeOptions};
use plot_one::PlotOne;
use std::sync::{Arc, RwLock};
use egui_plot :: {Line, Plot};
use std::thread;
use std::time::Duration;


struct MyApp {
    plot_one: Arc<RwLock<PlotOne>>,
    plot_two: Arc<RwLock<PlotOne>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            plot_one: Arc::new(RwLock::new(PlotOne::new())),
            plot_two: Arc::new(RwLock::new(PlotOne::new())),
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let app = MyApp::default();
    let plot_one_accessor = app.plot_one.clone();
    let plot_two_accessor = app.plot_two.clone();

    let mut count: f64 = 0.0;
    let sleep_time = 50;
    thread::spawn(move || {
        loop{
            count += 1.0;
            plot_one_accessor.write().unwrap().append_value(count, count + 1.0);
            thread::sleep(Duration::from_millis(sleep_time));
        }
    });

    thread::spawn(move || {
        loop{
            count += 1.0;
            plot_two_accessor.write().unwrap().append_value(count + 4.0, count + 4.0);
            thread::sleep(Duration::from_millis(sleep_time));
        }
    });

    let native_options = NativeOptions{
        initial_window_size: Some(egui::vec2(960.0, 720.0)),
        ..Default::default()
    };


    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(app)}),
    )
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            let plot = Plot::new("measurements");
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.plot_one.read().unwrap().get_values()));//reading from the rawData vector, use read() method with get_values()
                plot_ui.line(Line::new(self.plot_two.read().unwrap().get_values()));
            });
        });
        ctx.request_repaint();
    }
}