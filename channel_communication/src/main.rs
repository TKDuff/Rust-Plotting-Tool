mod plot_one;

//use eframe::{egui, NativeOptions};
use plot_one::PlotOne;
use std::sync::{Arc, RwLock};
//use egui_plot :: {Line, Plot};
use std::thread;
use std::time::Duration;
use crossbeam::channel::unbounded;
use std::time::Instant;


struct MyApp {
    plot_one: Arc<RwLock<PlotOne>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            plot_one: Arc::new(RwLock::new(PlotOne::new())),
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let app = MyApp::default();
    let plot_one_accessor = app.plot_one.clone();
    let (sender, receiver) = unbounded();


    let mut count: f64 = 0.0;
    let sleep_time = 10;


        let handle = thread::spawn(move || {
            //let mut chunk = vec![];
            let mut chunk: Vec<[f64; 2]> = Vec::new();
            
            loop{
                let mut writer = plot_one_accessor.write().unwrap();
                count += 1.0;   
                writer.append_value(count, count + 1.0);

                if count % 10000000.0 == 0.0 {
                    let send_time = Instant::now(); // Time of sending
                    chunk = writer.get_chunk();
                    sender.send((chunk, send_time)).unwrap();
                }
                //thread::sleep(Duration::from_millis(sleep_time));
            }
        });

        let downsample = thread::spawn(move || {
            if let Ok((received_chunk, send_time)) = receiver.recv() {
                // Process the received chunk
                //println!("{:?}", received_chunk); // Placeholder for your downsampling logic
                
                let receive_time = Instant::now();
                let latency = receive_time.duration_since(send_time);
                println!("Method: Writer get chunk\nPoints: 10000000\nLatency: {:?}", latency);
            }
        });
    /*
    let native_options = NativeOptions{
        initial_window_size: Some(egui::vec2(960.0, 720.0)),
        ..Default::default()
    };


    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(app)}),
    )*/
    handle.join().unwrap();
    downsample.join().unwrap();
    Ok(())
}

/*
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            let plot = Plot::new("measurements");
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.plot_one.read().unwrap().get_values()));//reading from the rawData vector, use read() method with get_values()
            });
        });
        ctx.request_repaint();
    }
}*/