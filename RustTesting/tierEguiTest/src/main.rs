mod tier;
use std::sync::Arc;

use eframe::{egui, NativeOptions};
use egui::{Visuals};
use egui_plot :: {Line, Plot};
use egui::{Vec2};

use std::thread;
use std::time::Duration;

use eframe::epaint::mutex::RwLock;
use tier::Tier;

struct MyApp {
    t1: Arc<RwLock<Tier>>,
    t2: Arc<RwLock<Tier>>,
    t3: Arc<RwLock<Tier>>,
}

impl Default for MyApp {
    fn default() ->  Self {
        Self {
            t1: Arc::new(RwLock::new(Tier::new())),
            t2: Arc::new(RwLock::new(Tier::new())),
            t3: Arc::new(RwLock::new(Tier::new())),

        }
    }
}

fn main() {

    let my_app = MyApp::default();
    let t1_access = my_app.t1.clone();
    let t2_access = my_app.t2.clone();
    let t3_access = my_app.t3.clone();


    let historic_data_handle = thread::spawn(move || {
        let mut x_increment = 1.0;
        let nums: Vec<f64> = vec![
            3.0, 4.0, 5.0, 
            9.0, 12.0, 16.0, 
            17.0, 21.0, 23.0, 
            24.0,26.0,29.0,
            31.0, 33.0,38.0,
            39.0,42.0,45.0,
            47.0, 49.0, 52.0, 
            56.0, 59.0, 62.0, 
            66.0, 70.0, 72.0, 
            73.0, 75.0, 79.0, 
            81.0, 83.0, 85.0, 
            88.0, 92.0, 95.0,
            99.0, 102.0, 105.0,];
            
            for &num in nums.iter() {
                x_increment = t1_access.write().push_float(num, x_increment);

                if t1_access.read().get_length() == 5 {
                    println!("\nTIER 1 start");
                    let mut vec_len;
                    let t1_average;

                    {
                        let t1_lock = t1_access.read();
                        let vec_slice = &t1_lock.vec[1..t1_lock.vec.len() - 1];
                        t1_average = t1_lock.calculate_average(vec_slice);
                        vec_len = t1_lock.vec.len();
                    }
                    println!("Avg: {}", t1_average[1]);

                    {
                        let mut t1_write = t1_access.write();
                        t1_write.vec[0] = t1_average;
                        t1_write.vec.drain(1..vec_len - 1);
                    }

                    t2_access.write().vec.push(t1_average);
                    println!("Tier 1 drain\nt1: {:?}\nt2: {:?}\n", t1_access.read().get_y(), t2_access.read().get_y());


                }

                /*Keep in mind length first element of t2 is previous element of t3, thus subtract 1 from condition. I.E if merging when length 7, means every six bins added merge
                When plotting it appears as every 5 bins then on the sixth bin the merge occurs*/ 
                if t2_access.read().vec.len() == 4 {
                    println!("\nTIER 2 start");
                    let mut vec_len;
                    let t2_average;

                    {
                        let t2_lock = t2_access.read();
                        let vec_slice = &t2_lock.vec[1..t2_lock.vec.len()];
                        t2_average = t2_lock.calculate_average(vec_slice);
                        vec_len = t2_lock.vec.len();
                    }
                    println!("Avg: {}", t2_average[1]);

                    {
                        let mut t3_write = t3_access.write();
                        t3_write.vec.push(t2_average);
                    }

                    {
                        let mut t2_write = t2_access.write();
                        t2_write.vec[0] = t2_average;
                        
                        t2_write.vec.drain(1..vec_len);
                    }


                    {
                        let mut t1_write = t1_access.write();
                        t1_write.vec[0] = t2_average;
                    }


                    println!("Tier 2 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\n", t1_access.read().get_y(), t2_access.read().get_y(), t3_access.read().get_y());
                }


                thread::sleep(Duration::from_millis(2000));

            }
            
    });

    
    let native_options = NativeOptions{
        ..Default::default()
    };

    let _ = eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(my_app)}),
    );

    historic_data_handle.join();
    

}


impl eframe::App for MyApp {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
    egui::CentralPanel::default().show(ctx, |ui| { 
        ctx.set_visuals(Visuals::light());
        
        let t1_line = Line::new( self.t1.read().get_points()).width(2.0);
        let t2_line = Line::new(self.t2.read().get_points()).width(2.0);
        let t3_line = Line::new(self.t3.read().get_points()).width(2.0);

        let plot = Plot::new("plot")
        .min_size(Vec2::new(800.0, 600.0));

        plot.show(ui, |plot_ui| {
            plot_ui.line(t1_line);
            plot_ui.line(t2_line);
            plot_ui.line(t3_line);
        });
    });
    ctx.request_repaint();
    }
}