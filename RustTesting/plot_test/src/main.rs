mod measurements;

use eframe::{egui, NativeOptions};
use egui_plot :: {BoxElem, BoxPlot, BoxSpread, Legend, Line, Plot};
use measurements::Measurements;
use std::io::{self, BufRead};
use std::thread;
use std::sync::{Arc, Mutex};    //'Arc' allows multiple threads to share the ownership of mutex across threads



/*create struct for the app, as of now contains points to plot
*/
struct MyApp { 
    measurements: Arc<Mutex<Measurements>>, //measurments module
}


/*'Default' is a trait containg the method default() 
default() assigns default values for a type automatically without needing to explicitaly say the type */
impl Default for MyApp {
    fn default() -> Self {  //returns instance of MyApp
        /*Self {} is the same as MyApp {} , the line below is just initialsing the struct values, point in this case*/
        Self {
            measurements: Arc::new(Mutex::new(Measurements::new(200.0))),
        }
    }
}

fn main() -> Result<(), eframe::Error> {

    let app = MyApp::default();
    //create clonse of app.measurments to access it via mutex
    let measurements_for_thread = app.measurements.clone();

    
    thread::spawn(move ||{
        let stdin = io::stdin();          //global stdin instance
        let locked_stdin = stdin.lock();  //lock stdin for exclusive access

        for line in locked_stdin.lines() {
            let line_string = line.unwrap();
            measurements_for_thread.lock().unwrap().append_str(&line_string);
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


        let box_plot = BoxPlot::new(vec![
            BoxElem::new(3.0, BoxSpread::new(1.5, 2.2, 2.5, 2.6, 3.1))
            .name("Data Set")
            .box_width(0.25),
        ])
        .name("Box plot demo");

        Plot::new("measurements")
        .legend(Legend::default())
        .show(ui, |plot_ui| plot_ui.box_plot(box_plot));

            /*
            let plot = Plot::new("measurements");
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.measurements.lock().unwrap().get_values()));
            });*/
        });
        ctx.request_repaint();  //repaint GUI without needing event
    }

}