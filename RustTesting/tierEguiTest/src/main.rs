mod tier;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use eframe::{egui, NativeOptions};
use eframe::epaint::mutex::RwLock;
use egui::{Vec2, Visuals};
use egui_plot::{Line, Plot, Points, PlotPoints, MarkerShape};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use tier::Tier;


struct MyApp {
    t1: Arc<RwLock<Tier>>,
    t2: Arc<RwLock<Tier>>,
    t3: Arc<RwLock<Tier>>,
    t4: Arc<RwLock<Tier>>,
    t5: Arc<RwLock<Tier>>,
}

impl Default for MyApp {
    fn default() ->  Self {
        Self {
            t1: Arc::new(RwLock::new(Tier::new())),
            t2: Arc::new(RwLock::new(Tier::new())),
            t3: Arc::new(RwLock::new(Tier::new())),
            t4: Arc::new(RwLock::new(Tier::new())),
            t5: Arc::new(RwLock::new(Tier::new())),

        }
    }
}

fn main() {

    let my_app = MyApp::default();
    let t1_access = my_app.t1.clone();
    let t2_access = my_app.t2.clone();
    let t3_access = my_app.t3.clone();
    let t4_access = my_app.t4.clone();
    let t5_access = my_app.t5.clone();

    //t3_access.write().vec.push([0.0, 0.0]);
    t2_access.write().vec.push([0.0, 0.0]);
    t1_access.write().vec.drain(0..1);
    t5_access.write().vec.drain(0..1);

    let mut t1_count = 0;
    let mut t2_count = 0;
    let mut t3_count = 0;
    

    let historic_data_handle = thread::spawn(move || {
        let mut x_increment = 1.0;
        let mut nums: Vec<f64> = Vec::new();

        let file = File::open("tierTest1.txt").expect("Unable to open file");
        let reader = io::BufReader::new(file);

        // Read lines into nums vector.
        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            match line.trim().parse::<f64>() {
                Ok(num) => nums.push(num),
                Err(_) => eprintln!("Warning: Line not a valid float, skipping"),
            }
        }
            
            for &num in nums.iter() {
                x_increment = t1_access.write().push_float(num, x_increment);
                println!("{}", num);
                if t1_access.read().get_length() == 3 {
                    //println!("TIER 1 start {}", t1_count);
                    t1_count +=1;

                    
                    let mut vec_len;
                    let t1_average;
                    let t1_last_elem;

                    {
                        let t1_lock = t1_access.read();
                        let vec_slice = &t1_lock.vec[0..t1_lock.vec.len() - 1];
                        t1_average = t1_lock.calculate_average(vec_slice);
                        vec_len = t1_lock.vec.len();
                    }
                    //println!("Avg: {}", t1_average[1]);

                    {
                        let mut t1_write = t1_access.write();
                        t1_write.vec.drain(0..vec_len - 1);
                        t1_last_elem = t1_write.vec[0];
                    }

                    {
                        let mut t2_write = t2_access.write();
                        let length = t2_write.vec.len() - 1;
                        t2_write.vec[length] = t1_average;
                        t2_write.vec.push(t1_last_elem);
                    }
                    //println!("Tier 1 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\nt4: {:?}\n", t1_access.read().get_y(), t2_access.read().get_y(),t3_access.read().get_y(), t4_access.read().get_y());        

                }

                /*Keep in mind length first element of t2 is previous element of t3, thus subtract 1 from condition. I.E if merging when length 7, means every six bins added merge
                When plotting it appears as every 5 bins then on the sixth bin the merge occurs*/
                if t2_access.read().vec.len() == 5 {
                    //println!("TIER 2 {}", t2_count);
                    process_tier(&t2_access, &t3_access);
                    //println!("Tier 2 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\nt4: {:?}\n", t1_access.read().get_y(), t2_access.read().get_y(), t3_access.read().get_y(), t4_access.read().get_y());
                }

                if t3_access.read().vec.len() == 5 {
                    println!("Pre Tier 3 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\nt4: {:?}\nt5: {:?}\n\n", t1_access.read().get_y(), t2_access.read().get_y(), t3_access.read().get_y(), t4_access.read().get_y(), t5_access.read().get_y());
                    process_tier(&t3_access, &t4_access);
                    println!("Tier 3 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\nt4: {:?}\nt5: {:?}\n\n", t1_access.read().get_y(), t2_access.read().get_y(), t3_access.read().get_y(), t4_access.read().get_y(), t5_access.read().get_y());
                }

                
                if t4_access.read().vec.len() == 5 {
                    println!("Pre Tier 4 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\nt4: {:?}\nt5: {:?}\n\n", t1_access.read().get_y(), t2_access.read().get_y(), t3_access.read().get_y(), t4_access.read().get_y(), t5_access.read().get_y());
                    process_tier(&t4_access, &t5_access);
                    println!("Tier 4 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\nt4: {:?}\nt5: {:?}\n\n", t1_access.read().get_y(), t2_access.read().get_y(), t3_access.read().get_y(), t4_access.read().get_y(), t5_access.read().get_y());
                }

                
                if t5_access.read().vec.len() == 9 {
                    let merged_t5_last_element = t5_access.write().merge_final_tier_vector_bins(3);
                    println!("Got the point {:?}", merged_t5_last_element);
                    println!("The first elem of t4 was {:?}", t4_access.read().vec[0]);
                    t4_access.write().vec[0] = merged_t5_last_element;
                    println!("Now the first elem of t4 is {:?}", t4_access.read().vec[0]);
                }


                pub fn process_tier(current_tier: &Arc<RwLock<Tier>>, previous_tier: &Arc<RwLock<Tier>>) {
                    let mut vec_len;
                    let current_tier_average;

                    {
                        let mut current_tier_lock = current_tier.write();
                        let vec_slice = &current_tier_lock.vec[1..current_tier_lock.vec.len()-1];
                        current_tier_average = current_tier_lock.calculate_average(vec_slice);
                        vec_len = current_tier_lock.vec.len();

                        current_tier_lock.vec[0] = current_tier_average;
                        current_tier_lock.vec.drain(1..vec_len-1);
                    }

                    {
                        let mut previous_tier_lock = previous_tier.write();
                        previous_tier_lock.vec.push(current_tier_average);
                    }
                }           

                thread::sleep(Duration::from_millis(10));

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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| { 
            ctx.set_visuals(Visuals::light());

            let t1_line = Line::new( self.t1.read().get_points()).width(2.0).color(egui::Color32::RED);
            let t2_line = Line::new(self.t2.read().get_points()).width(2.0).color(egui::Color32::BLUE);
            let t3_line = Line::new(self.t3.read().get_points()).width(2.0).color(egui::Color32::GREEN);
            let t4_line = Line::new(self.t4.read().get_points()).width(2.0).color(egui::Color32::BROWN);
            let t5_line = Line::new(self.t5.read().get_points()).width(2.0).color(egui::Color32::BLACK);
            
            
            let plot = Plot::new("plot")
            .min_size(Vec2::new(800.0, 600.0));
        
        plot.show(ui, |plot_ui| {
            plot_ui.line(t1_line);
            plot_ui.line(t2_line);
            plot_ui.line(t3_line);
            plot_ui.line(t4_line);
            plot_ui.line(t5_line);
        });
    });
    ctx.request_repaint();
    }
}

//T1 - 4
//T2 - 8
//T3 - 12 - off by 3 so 9

//Each tier has reference higher tier first element and lower tier last element
//Not a good approach, see each tier stretches into another

/*
Working version for now, best solution is middle of emptied tier is never empty, contain reference to last point of previous and first point of next
Problem exist with t2...
* In all tiers last point is the merged value of the next tier
* For t2, the last point is the current value of t1. The actual merrged t1 value (the real last element of t2) is the second last point 

So is t1 was [35,39,37,24] upon merge beocomes
[t1]: 24
t2: [37, 24]


See, 35,39,37 merged become 37. however final point of t2 is 24. 
Problem arise out of fact that all tiers beside t1 will contain two points at minimum, however t1 will always contain a single point.
t1 always contain newest point, all other tiers contain previous tier last value and current merged. 

Since t1 contain only next value, in order for t2 line plot to conncect to t1, must have reference to t2 single valuw
*/
