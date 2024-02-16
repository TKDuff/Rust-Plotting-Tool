mod tier;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool,Ordering};
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
    rd: Arc<RwLock<Tier>>,
    t1: Arc<RwLock<Tier>>,
    t2: Arc<RwLock<Tier>>,
    t3: Arc<RwLock<Tier>>,
    should_halt: Arc<AtomicBool<>>
}

impl Default for MyApp {
    fn default() ->  Self {
        Self {
            rd: Arc::new(RwLock::new(Tier::new())),
            t1: Arc::new(RwLock::new(Tier::new())),
            t2: Arc::new(RwLock::new(Tier::new())),
            t3: Arc::new(RwLock::new(Tier::new())),
            should_halt: Arc::new(AtomicBool::new(false)),


        }
    }
}

fn main() {

    let my_app = MyApp::default();
    let rd_acess = my_app.rd.clone();
    let t1_access = my_app.t1.clone();
    let t2_access = my_app.t2.clone();
    let t3_access = my_app.t3.clone();
    let should_halt_clone = my_app.should_halt.clone();

    //t2_access.write().vec.push([0.0, 0.0]);
    t1_access.write().vec.push([0.0, 0.0]);
    rd_acess.write().vec.drain(0..1);
    t3_access.write().vec.drain(0..1);

    

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
                if !should_halt_clone.load(Ordering::SeqCst) {
                x_increment = rd_acess.write().push_float(num, x_increment);
                //println!("{}", num);
                if rd_acess.read().get_length() == 5 {
                    //println!("TIER 1 start {}", rd_count);

                    
                    let mut vec_len;
                    let rd_average;
                    let rd_last_elem;

                    {
                        let rd_lock = rd_acess.read();
                        let vec_slice = &rd_lock.vec[0..rd_lock.vec.len() - 1];
                        rd_average = rd_lock.calculate_average(vec_slice);
                        vec_len = rd_lock.vec.len();
                    }
                    //println!("Avg: {}", rd_average[1]);

                    {
                        let mut rd_write = rd_acess.write();
                        rd_write.vec.drain(0..vec_len - 1);
                        rd_last_elem = rd_write.vec[0];
                    }

                    {
                        let mut t1_write = t1_access.write();
                        let length = t1_write.vec.len() - 1;
                        t1_write.vec[length] = rd_average;
                        t1_write.vec.push(rd_last_elem);
                    }
                    println!("Raw data Merge\nrd: {:?}\nt1: {:?}\nt2: {:?}\nt3: {:?}\n", rd_acess.read().get_y(), t1_access.read().get_y(),t2_access.read().get_y(), t3_access.read().get_y());        

                }

                /*Keep in mind length first element of t1 is previous element of t2, thus subtract 1 from condition. I.E if merging when length 7, means every six bins added merge
                When plotting it appears as every 5 bins then on the sixth bin the merge occurs*/
                if t1_access.read().vec.len() == 5 {
                    //println!("TIER 2 {}", t1_count);
                    process_tier(&t1_access, &t2_access);
                    println!("Tier 1 drain\nrd: {:?}\nt1: {:?}\nt2: {:?}\nt3: {:?}\n", rd_acess.read().get_y(), t1_access.read().get_y(), t2_access.read().get_y(), t3_access.read().get_y());
                }

                if t2_access.read().vec.len() == 5 {
                    process_tier(&t2_access, &t3_access);
                    println!("Tier 2 drain\nrd: {:?}\nt1: {:?}\nt2: {:?}\nt3: {:?}\n", rd_acess.read().get_y(), t1_access.read().get_y(), t2_access.read().get_y(), t3_access.read().get_y());
                }

                
                if t3_access.read().vec.len() == 5 {
                    let merged_t3_last_element = t3_access.write().merge_final_tier_vector_bins(3);
                    println!("Got the point {:?}", merged_t3_last_element);
                    println!("The first elem of t2 was {:?}", t2_access.read().vec[0]);
                    t2_access.write().vec[0] = merged_t3_last_element;
                    println!("Now the first elem of t2 is {:?}", t2_access.read().vec[0]);
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

                thread::sleep(Duration::from_millis(50));
            }

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

            let rd_line = Line::new( self.rd.read().get_points()).width(2.0).color(egui::Color32::RED);
            let t1_line = Line::new(self.t1.read().get_points()).width(2.0).color(egui::Color32::BLUE);
            let t2_line = Line::new(self.t2.read().get_points()).width(2.0).color(egui::Color32::GREEN);
            let t3_line = Line::new(self.t3.read().get_points()).width(2.0).color(egui::Color32::BROWN);

            if ui.button("Halt Processing").clicked() {
                self.should_halt.store(true, Ordering::SeqCst);
            }
            
            
            let plot = Plot::new("plot")
            .min_size(Vec2::new(800.0, 600.0));
        
        plot.show(ui, |plot_ui| {
            plot_ui.line(rd_line);
            plot_ui.line(t1_line);
            plot_ui.line(t2_line);
            plot_ui.line(t3_line);
        });
    });
    ctx.request_repaint();
    }
}

//rd - 4
//t1 - 8
//t2 - 12 - off by 3 so 9

//Each tier has reference higher tier first element and lower tier last element
//Not a good approach, see each tier stretches into another

/*
Working version for now, best solution is middle of emptied tier is never empty, contain reference to last point of previous and first point of next
Problem exist with t1...
* In all tiers last point is the merged value of the next tier
* For t1, the last point is the current value of rd. The actual merrged rd value (the real last element of t1) is the second last point 

So is rd was [35,39,37,24] upon merge beocomes
[rd]: 24
t1: [37, 24]


See, 35,39,37 merged become 37. however final point of t1 is 24. 
Problem arise out of fact that all tiers beside rd will contain two points at minimum, however rd will always contain a single point.
rd always contain newest point, all other tiers contain previous tier last value and current merged. 

Since rd contain only next value, in order for t1 line plot to conncect to rd, must have reference to t1 single valuw
*/
