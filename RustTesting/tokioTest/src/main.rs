#![allow(warnings)] //Remove warning, be sure to remove this
mod data_module;
use data_module::{StdinData, DownsampledData};
use tokio::runtime::Runtime;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::time::{self, Duration, Interval};
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

fn main() -> Result<(), eframe::Error> {
    let (rd_sender, hd_receiver) = channel::unbounded();
    //let (hd_sender, rd_receiver) = channel::unbounded();
    let (hd_sender, mut rd_receiver) = tokio::sync::mpsc::unbounded_channel::<(f64, f64)>();


    let my_app = MyApp::default();
    let raw_data_thread = my_app.raw_data.clone();


    let rt = Runtime::new().unwrap();

    rt.spawn(async move {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        let mut interval = time::interval(Duration::from_secs(1));
        let mut line_count = 0;
        let  points_count = 10;
        let mut length = 0;
        /*
        In this loop, make use of the select! macro, allows to wait on multiple asynchronous operations simultaneously and proceed with the one that completes first
        Contains branches, like pattern matching, denoted by the =>, so "line = lines.next_line() => {" and "_ = interval.tick() => {" are 2 branches waiting on

        _ = interval.tick() - interval creates an async stream that yields (returns) fixed time intervals, which is time duration between each tick of the stream.
                            - tick() method runs task when tick occurs, so print count of lines appended every tick (every second)

        "point_means_result = rd_receiver.recv() => {" - when receive message to remove chunk, remove chunk

        All branches polled continously in loop, whichever receive return value first is exectuted first, this done for 2 reasons
        1) Non-blocking IO - previously deleting a chunk was dependant on adding a new line. When the incoming data rate was slow
         when a r.d chunk was aggregated, the historic line would be drawn over it until a new point was added to r.d which allows the if condition to remove the chunk

        2)Tokio intervals can be used to record time asynchronously, thus can record the number of lines being read per second
         */
        loop {
            tokio::select! {
                line = lines.next_line() => {
                    if let Ok(Some(line)) = line {
                        println!("{}", line);
                        raw_data_thread.write().unwrap().append_str(&line);
                        line_count += 1;
                        length = raw_data_thread.read().unwrap().get_length();
                        if length % points_count == 0 {
                            rd_sender.send((length, points_count)).unwrap();
                        }
                    } else {
                        // End of input or an error. You can break or handle it as needed.
                        break;
                    }
                },
                point_means_result = rd_receiver.recv() => {
                    if let Some(point_means) = point_means_result {
                        println!("Should remove {:?}", point_means);
                        // Now that we have point_means, we can use it
                        raw_data_thread.write().unwrap().remove_chunk(points_count, point_means);
                    }
                },  
                _ = interval.tick() => {
                    //println!("Lines added in last second {}", line_count);
                    line_count = 0;
                },
            }
        }
    });

    
    let downsampler_raw_data_thread = my_app.raw_data.clone();
    let downsampler_thread = my_app.historic_data.clone();

    
    let historic_data_handle = thread::spawn(move || {
        let mut chunk: Vec<[f64;2]>;
        let mut objective_length = 0;
        
        for message in hd_receiver {
            let(raw_data_length, point_count) = message;
            chunk = downsampler_raw_data_thread.read().unwrap().get_chunk(point_count);
            hd_sender.send(downsampler_thread.write().unwrap().append_statistics(chunk, point_count));
            /*
            objective_length += 1;
            if objective_length % 4 == 0 {
                downsampler_thread.write().unwrap().combineBins();
            }*/
        }
    });

    let native_options = NativeOptions{
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(my_app)}),
    );

    //raw_data_handle.join().unwrap();
    //historic_data_handle.join().unwrap();

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


//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/RustTesting/tokioTest/ && cargo run --bin tokioTest)