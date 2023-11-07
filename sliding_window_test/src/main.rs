mod raw_data;

use eframe::{egui, NativeOptions};
use egui_plot :: {Line, Plot};
use raw_data::RawData;
use std::io::{self, BufRead};
use std::thread;
use std::sync::{mpsc, Arc, RwLock}; // Change Mutex to RwLock here


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
    let (tx, rx) = mpsc::channel(); 
    //create clonse of app.measurments to access it via mutex


    let raw_data_thread = app.raw_data.clone();
    let raw_data_clone_for_gui = app.raw_data.clone();

    thread::spawn(move ||{
        let stdin = io::stdin();          //global stdin instance
        let locked_stdin = stdin.lock();  //lock stdin for exclusive access
        let mut lines_iterator = locked_stdin.lines();

        loop {
            match rx.try_recv() {
                Ok((downsampled_data, start_range, end_range)) => {
                    // If data is received, process it
                    raw_data_thread.write().unwrap().amend(&downsampled_data, start_range, end_range);
                },
                Err(mpsc::TryRecvError::Empty) => {
                    // No data received, nothing to do here
                },Err(mpsc::TryRecvError::Disconnected) => {
                    // The sending side of the channel has been closed, handle as needed
                    break;
                }
            }

            match lines_iterator.next() {
                Some(Ok(line)) => {
                    raw_data_thread.write().unwrap().append_str(&line);
                },
                Some(Err(_e)) => {
                    eprint!("Error")
                },
                None => {
                }
            }
        }
    });

    let tx_clone = tx.clone(); 
    thread::spawn(move ||{
        let mut prev_count = 0; 
        loop {
            let current_count = app.raw_data.read().unwrap().get_length();
            if current_count % 100 ==0 && current_count != prev_count{   
                /*downsampling done here, use channel to send the downsampled value */
                let previous_ten_index = current_count - 100;
                let to_downsample = app.raw_data.read().unwrap().get_previous_ten(current_count, previous_ten_index);
                
                let downsampled: Vec<_> = to_downsample.into_iter()
                .enumerate()
                    .filter_map(|(index, value)| {
                    if index % 2 == 0 { Some(value) } else { None }
                }).collect();

                tx_clone.send((downsampled, previous_ten_index ,current_count)).expect("Failed to send");
                prev_count = current_count;
            };
        }
    });

    
    let nativ_options = NativeOptions{
        initial_window_size: Some(egui::vec2(960.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native("App", nativ_options, Box::new(move |_|{Box::new(MyApp {
        raw_data: raw_data_clone_for_gui,})}),);
    
    Ok(())
    
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
