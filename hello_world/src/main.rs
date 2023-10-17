//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {

    /*Window configurations */
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
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

struct MyApp {
    name: String,   //name to display
}

//'Default' trait to set initial values for MyApp struct fields 
impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {  //lambda function, 'ui' as argument, here is where the actual UI is designed
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.label(format!("Hello '{}'", self.name));
        });
    }
}
