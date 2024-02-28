mod data_module;
use data_module::{StdinData, DownsampledData};
use eframe::egui::PointerState;



use std::any::Any;
use std::collections::btree_map::Values;
use std::fmt::format;
use std::thread;
use eframe::{egui, NativeOptions}; 
use egui_plot :: {BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner, Legend, Line, MarkerShape, Plot, PlotItem, PlotPoint, Points, Text, PlotResponse};
use egui::{CentralPanel, Id, Pos2, Vec2, Visuals};
use std::io::{self, BufRead, SeekFrom};
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
    // let (rd_sender, hd_receiver) = channel::unbounded();
    // let (hd_sender, rd_receiver) = channel::unbounded();

    let my_app = MyApp::default();

    /*
    //let sender_clone = sender.clone();
    let raw_data_thread = my_app.raw_data.clone();

    let raw_data_handle = thread::spawn(move || { 
        let stdin = io::stdin();          //global stdin instance
        let locked_stdin = stdin.lock();  //lock stdin for exclusive access
        let mut length = 0;
        let mut points_count = 30;

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
        let lltb_points = 14;
        
        for message in hd_receiver {
            let(raw_data_length, point_count) = message;
            chunk = downsampler_raw_data_thread.read().unwrap().get_chunk(point_count);
            hd_sender.send(downsampler_thread.write().unwrap().append_statistics(chunk, point_count));
            objective_length += 1;
            if objective_length % lltb_points == 0 {
                downsampler_thread.write().unwrap().combineBins(lltb_points);
            }
        }
    });*/

    
    let native_options = NativeOptions {
        //initial_window_size: Some(egui::vec2(1400.0, 700.0)), 
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(my_app)}),
    );

    // raw_data_handle.join().unwrap();
    // historic_data_handle.join().unwrap();

    Ok(())
}

impl eframe::App for MyApp {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
        egui::CentralPanel::default().show(ctx, |ui| { 
            ctx.set_visuals(Visuals::light());

            let values = self.raw_data.read().unwrap().get_values();
            let raw_plot_line = Line::new(values.clone()).color(egui::Color32::BLUE).name("Tier 1").id(egui::Id::new(1));

            let mut position: Option<PlotPoint> =None;
            let mut hovered_item: Option<String> = None;
            let id = ui.make_persistent_id("interaction_demo");


            let plot = Plot::new("plot").width(1300.0).height(600.0).legend(Legend::default()).id(id);
            

            ui.vertical(|ui| {


                let p_r: PlotResponse<()> = plot.show(ui, |plot_ui| {
                    plot_ui.line(raw_plot_line);  
                    position = plot_ui.pointer_coordinate();
                });
                let click = ctx.input(|i| i.pointer.any_click());

                if click {
                    find_closest(position, p_r.hovered_plot_item, &values)
                }
                
            });            
        }); 

        ctx.request_repaint();
    }
}

fn find_closest(position: Option<PlotPoint>, hovered: Option<Id>, values: &Vec<[f64; 2]>) {
    let hovered_line = hovered
        .and_then(|id| (0..1).find(|&i| id == egui::Id::new(i)))
        .map(|i| format!("Tier {}", i + 1))
        .unwrap_or_else(|| "None".to_string());



    if let Some(plot_point) = position {
        let x = plot_point.x;
        let y = plot_point.y;
        let tolerance = 0.1;

        // Find the closest point within the tolerance
        let closest = values.iter().find(|&&p| {
            (x - p[0]).abs() <= tolerance && (y - p[1]).abs() <= tolerance
        });

        println!("Position x: {:.2}, y: {:.2}", x, y);
        println!("Tier {}", hovered_line);

        if let Some(closest_point) = closest {
            println!("Closest point found at {:?}", closest_point);
        } else {
            println!("No point found within tolerance");
        }
    } else {
        println!("No position provided");
    }
}