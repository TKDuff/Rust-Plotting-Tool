mod raw_data;

extern crate statrs;

use eframe::{egui, NativeOptions};
use egui_plot :: {Line, Plot};
use raw_data::RawData;
use std::io::{self, BufRead};
use std::thread;
use std::sync::{Arc, RwLock}; // Change Mutex to RwLock here
use crossbeam::channel::unbounded;
use statrs::statistics::{OrderStatistics, Min, Max};
use statrs::statistics::Data;






/*create struct for the app, as of now contains points to plot*/
struct MyApp { 
    raw_data: Arc<RwLock<RawData>>, //measurments module
}


/*'Default' is a trait containg the method default() 
default() assigns default values for a type automatically without needing to explicitaly say the type */
impl Default for MyApp {
    fn default() -> Self {  //returns instance of MyApp
        /*Self {} is the same as MyApp {} , the line below is just initialsing the struct values, point in this case*/
        Self {
            raw_data: Arc::new(RwLock::new(RawData::new(200.0))),

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
        let mut chunk: Vec<[f64; 2]> = Vec::new();

        loop {
            if let Ok(points) = receiver.recv() {
                chunk = raw_data_for_downsampler.read().unwrap().get_chunk(points);
                let x_for_quartile: Vec<f64> = chunk.iter().map(|&[val, _]| val).collect();
                let y_for_quartile: Vec<f64> = chunk.iter().map(|&[_, val]| val).collect();

                println!("\n{:?}\n\n{:?}", x_for_quartile, y_for_quartile);
                let mut x = Data::new(x_for_quartile);
                let mut y = Data::new(y_for_quartile);



                println!("X:\nLower Quartile: {}\nUpper Quartile: {}\nMedian: {}\nMin: {}\nMax: {}",x.lower_quartile(), x.upper_quartile(), x.median(), x.min(), x.max());
                println!("\nY:\nLower Quartile: {}\nUpper Quartile: {}\nMedian: {}\nMin: {}\nMax: {}",y.lower_quartile(), y.upper_quartile(), y.median(), x.min(), x.max());
                //let quartiles = Quartiles::new
            }   
        }

        /*
        let mut prev_count = 0; 
        loop {
            let current_count = app.raw_data.read().unwrap().get_length();
            if current_count % 20 ==0 && current_count != prev_count{   
                /*downsampling done here, use channel to send the downsampled value */
                let previous_ten_index = current_count - 20;
                let to_downsample = app.raw_data.read().unwrap().get_previous_ten(current_count, previous_ten_index);
                
                let downsampled: Vec<_> = to_downsample.into_iter()
                .enumerate()
                    .filter_map(|(index, value)| {
                    if index % 2 == 0 { Some(value) } else { None }
                }).collect();

                tx_clone.send((downsampled, previous_ten_index ,current_count)).expect("Failed to send");
                prev_count = current_count;
            };
        }*/
    });

    /*
    let nativ_options = NativeOptions{
        initial_window_size: Some(egui::vec2(960.0, 720.0)),
        ..Default::default()
    };*/

    /*
    eframe::run_native("App", nativ_options, Box::new(move |_|{Box::new(MyApp {
        raw_data: raw_data_for_egui,})}),);
    */
    writer.join().unwrap();
    downsample.join().unwrap();
    Ok(())
    
}

/*
impl eframe::App for MyApp {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
        egui::CentralPanel::default().show(ctx, |ui| { 


            let plot = Plot::new("measurements").allow_zoom(true);
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.raw_data.read().unwrap().get_values()));//reading from the rawData vector, use read() method with get_values()
            });
        });
        ctx.request_repaint();  //repaint GUI without needing event
    }
}*/