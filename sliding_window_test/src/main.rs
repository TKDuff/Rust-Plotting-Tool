mod raw_data;

extern crate statrs;

use eframe::{egui, NativeOptions};
use egui_plot :: {BoxElem, BoxPlot, BoxSpread, Legend, Line, Plot};
use raw_data::{RawData, Statistics};
use std::io::{self, BufRead};
use std::thread;
use std::sync::{Arc, RwLock}; // Change Mutex to RwLock here
use crossbeam::channel::unbounded;
use statrs::statistics::{OrderStatistics, Min, Max, Distribution};
use statrs::statistics::Data;

/*create struct for the app, as of now contains points to plot*/
struct MyApp { 
    raw_data: Arc<RwLock<RawData>>, //measurments module
    plotType: String,
}


/*'Default' is a trait containg the method default() 
default() assigns default values for a type automatically without needing to explicitaly say the type */
impl Default for MyApp {
    fn default() -> Self {  //returns instance of MyApp
        /*Self {} is the same as MyApp {} , the line below is just initialsing the struct values, point in this case*/
        Self {
            raw_data: Arc::new(RwLock::new(RawData::new())),
            plotType: "Box Plot".to_string(),
        }
    }
}


fn main() -> Result<(), eframe::Error> {
    let app = MyApp::default();
    let (sender, receiver) = unbounded();
    //create clonse of app.measurments to access it via mutex


    let raw_data_thread = app.raw_data.clone();
    let raw_data_for_downsampler = app.raw_data.clone();
    let raw_data_for_egui = app.raw_data.clone();
    let boxType_for_egui =app.plotType.clone();

    let writer = thread::spawn(move ||{
        let stdin = io::stdin();          //global stdin instance
        let locked_stdin = stdin.lock();  //lock stdin for exclusive access
        let mut count = 0;
        let points = 50;


        for line in locked_stdin.lines() {
            let line_string = line.unwrap();
            raw_data_thread.write().unwrap().append_str(&line_string);
            count+= 1;
            if count % points == 0 {
                sender.send(points).unwrap(); 
            }

        }
    });

    
    let downsample = thread::spawn(move ||{
        let mut chunk: Vec<Statistics> = Vec::new();

        loop {
            if let Ok(points) = receiver.recv() {

                let (chunk, length) = raw_data_for_downsampler.read().unwrap().get_chunk(points); 
                
                //let x_for_quartile: Vec<f64> = chunk.iter().map(|&[val, _]| val).collect();
                let x_for_quartile: Vec<f64> = chunk.iter().map(|stat| stat.x_avg).collect();

                let y_for_quartile: Vec<f64> = chunk.iter().map(|stat| stat.y_avg).collect();

                
                println!("\n{:?}\n\n{:?}", x_for_quartile, y_for_quartile);
                
                let mut x = Data::new(x_for_quartile);
                let mut y = Data::new(y_for_quartile);



                println!("X:\nLower Quartile: {}\nUpper Quartile: {}\nMedian: {}\nMin: {}\nMax: {}",x.lower_quartile(), x.upper_quartile(), x.median(), x.min(), x.max());
                println!("\nY:\nLower Quartile: {}\nUpper Quartile: {}\nMedian: {}\nMin: {}\nMax: {}",y.lower_quartile(), y.upper_quartile(), y.median(), x.min(), x.max());

                let singleStat = Statistics {
                    x_avg: x.median(),
                    x_lower_quartile: x.lower_quartile(),
                    x_upper_quartile: x.upper_quartile(),
                    x_median: x.median(),
                    x_min: x.min(),
                    x_max: x.max(),

                    y_avg: y.median(),
                    y_lower_quartile: y.lower_quartile(),
                    y_upper_quartile: y.upper_quartile(),
                    y_median: y.median(),
                    y_min: y.min(),
                    y_max: y.max(),
                };
                //let quartiles = Quartiles::new
                raw_data_for_downsampler.write().unwrap().append_chunk(singleStat.clone(), length);
                raw_data_for_downsampler.write().unwrap().append_box_plot(singleStat);
            }   
        }
    });

    
    let nativ_options = NativeOptions{
        initial_window_size: Some(egui::vec2(960.0, 720.0)),
        ..Default::default()
    };

    
    eframe::run_native("App", nativ_options, Box::new(move |_|{Box::new(MyApp {
        raw_data: raw_data_for_egui, plotType: boxType_for_egui})}),);
    
    writer.join().unwrap();
    downsample.join().unwrap();
    Ok(())
    
}


impl eframe::App for MyApp {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
        egui::CentralPanel::default().show(ctx, |ui| { 
            //let mut plot_type = "Box Plot";
            ui.radio_value(&mut self.plotType, "Box Plot".to_string(), "Box Plot");
            ui.radio_value(&mut self.plotType, "Line Plot".to_string(), "Line Plot");

            let box_plot_stats = self.raw_data.read().unwrap().get_box_plot();

            let x_box_plot = BoxPlot::new(vec![
                BoxElem::new(0.25, BoxSpread::new(box_plot_stats.x_min, box_plot_stats.x_lower_quartile, box_plot_stats.x_median, box_plot_stats.x_upper_quartile, box_plot_stats.x_max))
                .name("X_Box")
                .box_width(0.25),
            ]);

            let y_box_plot = BoxPlot::new(vec![
                BoxElem::new(0.25, BoxSpread::new(box_plot_stats.y_min, box_plot_stats.y_lower_quartile, box_plot_stats.y_median, box_plot_stats.y_upper_quartile, box_plot_stats.y_max))
                .name("X_Box")
                .box_width(0.25),
            ]);

            let plot_line = Line::new(self.raw_data.read().unwrap().get_plot_values());

            match self.plotType.as_str() {
                "Box Plot" => {
                    Plot::new("measurements")
                        .legend(Legend::default())
                        .show(ui, |plot_ui| {
                            plot_ui.box_plot(x_box_plot);
                            plot_ui.box_plot(y_box_plot);
                        });
                    }
                "Line Plot" => {
                    Plot::new("measurements")
                    .show(ui, |plot_ui| {
                        plot_ui.line(plot_line);
                    });

                }
                    _ => {}
            }


            
            
        });
        ctx.request_repaint();  //repaint GUI without needing event
    }
}