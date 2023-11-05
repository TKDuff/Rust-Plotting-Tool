mod raw_data;

use eframe::{egui, NativeOptions};
use egui_plot :: {Line, Plot};
use raw_data::rawData;
use std::io::{self, BufRead};
use std::thread;
//use std::sync::{Arc, Mutex};    //'Arc' allows multiple threads to share the ownership of mutex across threads
use std::sync::{Arc, RwLock}; // Change Mutex to RwLock here




/*create struct for the app, as of now contains points to plot
*/
struct MyApp { 
    raw_data: Arc<RwLock<rawData>>, //measurments module
}


/*'Default' is a trait containg the method default() 
default() assigns default values for a type automatically without needing to explicitaly say the type */
impl Default for MyApp {
    fn default() -> Self {  //returns instance of MyApp
        /*Self {} is the same as MyApp {} , the line below is just initialsing the struct values, point in this case*/
        Self {
            raw_data: Arc::new(RwLock::new(rawData::new(200.0))),
        }
    }
}


fn main() -> Result<(), eframe::Error> {

    let app = MyApp::default();
    //create clonse of app.measurments to access it via mutex
    let raw_data_thread = app.raw_data.clone();

    
    thread::spawn(move ||{
        let stdin = io::stdin();          //global stdin instance
        let locked_stdin = stdin.lock();  //lock stdin for exclusive access

        for line in locked_stdin.lines() {
            let line_string = line.unwrap();
            raw_data_thread.write().unwrap().append_str(&line_string); //write() method used here since using 'append_str()' method which modifies the vector values
        }
    });

    let nativ_options = NativeOptions{
        initial_window_size: Some(egui::vec2(960.0, 720.0)),
        ..Default::default()
    };

    
    eframe::run_native("App", nativ_options, Box::new(move |_|{Box::new(app)}),)
    
}

impl eframe::App for MyApp {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
        egui::CentralPanel::default().show(ctx, |ui| { 


            let plot = Plot::new("measurements");
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.raw_data.read().unwrap().get_values()));//reading from the rawData vector, use read() method with get_values()
            });
        });
        ctx.request_repaint();  //repaint GUI without needing event
    }
}
