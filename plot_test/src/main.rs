mod measurements;

use eframe::egui;
use egui_plot :: {Line, Plot};
use measurements::Measurements;


/*create struct for the app, as of now contains points to plot
*/
struct MyApp {
    points: Vec<[f64; 2]>,   //points to display
    measurements: Measurements,
}

impl MyApp {
    fn double_points(&mut self) {
        for point in &mut self.points {
            point[0] *= 3.0;
            point[1] *= 2.0;
        }
    }
}

/*'Default' is a trait containg the method default() 
default() assigns default values for a type automatically without needing to explicitaly say the type */
impl Default for MyApp {
    fn default() -> Self {  //returns instance of MyApp
        /*Self {} is the same as MyApp {} , the line below is just initialsing the struct values, point in this case*/
        Self {
            points: vec![],
            measurements: Measurements::new(),
        }
    }
}

fn main() -> Result<(), eframe::Error> {

    

    /*Window configurations */
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(960.0, 720.0)),
        ..Default::default()    //sets the default configurations using default() method
    };

    /*Calling run_native() method provided by eframe to actually run the app 
    Takes three arguments, app name, configuration options (defined above), lambda function setting up the application
    */
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| {  
            Box::<MyApp>::default()
        }),
    )
}

impl eframe::App for MyApp {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
        egui::CentralPanel::default().show(ctx, |ui| { 
            if ui.button("Double").clicked() {
                self.double_points();
            }

            let plot = Plot::new("measurements");
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.points.clone()));
            });
        });
    }
}
