mod tier;
use std::sync::Arc;

use eframe::epaint::Color32;
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
    t4: Arc<RwLock<Tier>>,
}

impl Default for MyApp {
    fn default() ->  Self {
        Self {
            t1: Arc::new(RwLock::new(Tier::new())),
            t2: Arc::new(RwLock::new(Tier::new())),
            t3: Arc::new(RwLock::new(Tier::new())),
            t4: Arc::new(RwLock::new(Tier::new())),

        }
    }
}

fn main() {

    let my_app = MyApp::default();
    let t1_access = my_app.t1.clone();
    let initial_tier:Arc<RwLock<Tier>>  = my_app.t2.clone();
    let t2_access:Arc<RwLock<Tier>>  = my_app.t2.clone();
    let t3_access = my_app.t3.clone();
    let t4_access = my_app.t4.clone();

    t3_access.write().vec.push([0.0, 0.0]);
    t1_access.write().vec.drain(0..1);

    let mut t1_count = 0;
    let mut t2_count = 0;
    let mut t3_count = 0;
    

    let historic_data_handle = thread::spawn(move || {
        let mut x_increment = 1.0;
        let nums: Vec<f64> = vec![

            
            41.30, 27.34, 39.68, 
            40.30, 42.04, 48.18, 
            36.58, 35.78, 47.42, 
            42.04, 49.67, 33.51, 
            43.91, 33.70, 48.52, 
            43.07, 39.61, 43.50, 
            24.78, 49.45, 42.86, 
            33.85, 42.12, 36.20, 
            46.38, 43.30, 43.97, 
            44.21, 38.24, 43.66, 
            36.24, 30.96, 32.71, 
            36.44, 32.91, 39.52, 
            34.73, 34.75, 34.05, 
            42.84, 34.18, 29.71, 
            41.22, 44.82, 31.87, 
            37.81, 42.90, 39.71, 
            32.51, 35.08, 34.35, 
            43.93, 41.36, 37.35, 
            44.68, 27.52, 39.01, 
            36.97, 36.27, 41.28, 
            47.06, 39.75, 39.57, 
            41.77, 37.89, 38.62, 
            41.64, 41.00, 32.92, 
            34.33, 48.42, 36.65, 
            44.38, 30.22, 38.95, 
            39.42, 34.71, 41.92, 
            39.45, 37.22, 33.95, 
            42.47, 46.57, 43.88, 
            45.70, 48.27, 45.69, 
            40.84, 39.94, 40.04, 
            36.86, 36.69, 35.60, 
            37.43, 35.58, 43.93, 
            32.88, 36.30, 39.11, 
            37.23, 34.59, 41.08, 
            46.06, 42.88, 40.22, 
            38.63, 50.65, 37.53, 
            42.11, 46.72, 49.50, 
            37.05, 43.03, 40.13, 
            42.54, 44.33, 46.32, 
            42.61, 46.01, 41.46, 
            37.43, 48.38, 45.25, 
            39.09, 50.28, 41.74, 
            37.77, 41.16, 40.31, 
            42.24, 45.72, 46.22, 
            44.22, 40.10, 43.76, 
            46.18, 47.70, 41.82, 
            39.96, 36.96, 35.22, 
            33.23, 33.37, 37.38, 
            42.93, 42.34, 41.33, 
            47.10, 42.41, 36.24, 
            39.54, 48.44, 42.94, 
            42.81, 39.49, 41.67, 
            39.81, 45.96, 39.88, 
            45.94, 41.92, 44.44, 
            39.82, 36.73, 40.07, 
            43.21, 42.02, 41.12, 
            33.26, 43.80, 32.12, 
            38.83, 34.55, 37.63, 
            40.72, 42.77, 49.00, 
            43.14, 42.18, 28.97, 
            37.54, 45.89, 36.02, 
            36.48, 33.25, 40.46, 
            36.53, 44.22, 37.66, 
            39.93, 49.54, 35.80, 
            40.83, 47.83, 37.89, 
            53.41, 46.54, 46.58, 
            44.43, 35.65, 35.96, 
            41.30, 43.83, 44.03, 
            47.99, 39.38, 43.68, 
            40.68, 34.36, 32.55, 
            33.12, 40.29, 32.48, 
            42.39, 45.34, 41.97, 
            47.57, 25.39, 35.79, 
            44.63, 43.67, 33.96, 
            44.05, 47.78, 44.45, 
            40.12, 36.19, 33.26, 
            44.47, 38.98, 53.60, 
            31.60, 39.40, 40.29, 
            37.85, 34.02, 42.75, 
            50.35, 29.86, 36.58, 
            34.59, 44.83, 44.52, 
            42.10, 40.49, 38.52, 
            44.25, 42.17, 45.29, 
            41.32, 35.43, 45.39, 
            41.55, 42.84, 48.04, 
            42.33, 38.64, 43.46, 
            38.70, 30.48, 35.34, 
            36.46, 30.61, 50.71, 
            36.93, 44.87, 35.28, 
            35.80, 31.07, 31.93, 
            43.33, 46.79, 33.18, 
            39.54, 39.65, 39.22, 
            41.72, 45.80, 33.17, 
            35.30, 42.01, 37.62, 
            42.53, 35.23, 39.95, 
            38.25, 41.97, 27.20, 
            46.18, 58.85, 44.75, 
            37.24, 43.27, 51.70, 
            33.06, 35.87, 46.46, 
            44.31, 32.25, 49.84, 
            39.95, 41.45, 38.58, 
            38.57, 43.12, 34.73, 
            44.62, 41.34, 47.93, 
            37.33, 41.18, 34.47, 
            45.07, 44.29, 35.37, 
            39.30, 43.59, 50.34, 
            41.94, 38.54, 46.27, 
            39.65, 37.48, 41.74, 
            38.19, 33.37, 38.30, 
            42.23, 44.17, 46.15, 
            37.61, 35.96, 41.76, 
            40.98, 38.13, 36.78, 
            37.74, 40.41, 34.37, 
            40.06, 40.76, 37.17, 
            38.80, 36.11, 34.69, 
            40.09, 36.52, 39.33, 
            42.84, 36.63, 43.00, 
            40.44, 40.12, 33.64, 
            42.83, 38.17, 44.10, 
            39.49, 42.29, 41.81, 
            42.68, 52.92, 44.04, 
            40.18, 50.26, 47.45, 
            37.62, 39.28, 41.03, 
            41.76, 41.05, 43.08, 
            39.10, 37.85, 35.21, 
            37.65, 40.45, 48.05, 
            40.93, 38.30, 44.88, 
            41.91, 36.83, 35.45, 
            40.51, 43.77, 46.01, 
            35.36, 40.18, 40.89, 
            35.68, 39.45, 43.40, 
            47.49, 50.38, 45.95, 
            38.61, 38.28, 44.21, 
            39.33, 41.78, 40.29, 
            41.64, 39.96, 42.57, 
            40.94, 45.96, 42.41, 
            39.39, 39.93, 40.63, 
            38.91, 39.40, 36.79, 
            37.16, 43.02, 38.58, 
            31.96, 32.36, 38.84, 
            51.62, 37.34, 41.62, 
            40.84, 38.22, 42.92, 
            43.00, 39.26, 39.97, 
            38.78, 41.72, 36.85, 
            39.89, 44.73, 41.45, 
            33.68, 38.58, 35.59, 
            33.45, 40.81, 42.09, 
            33.84, 36.22, 40.90, 
            41.19, 45.64, 43.60, 
            35.80, 36.70, 38.68, 
            42.18, 44.88, 37.40, 
            41.44, 40.20, 42.97, 
            35.22, 42.64, 41.80, 
            33.30, 39.71, 43.34, 
            43.55, 40.65, 37.86, 
            41.26, 41.54, 41.58, 
            36.55, 35.76, 33.91, 
            39.94, 43.76, 32.44, 
            42.86, 40.94, 47.88, 
            42.70, 33.47, 35.30, 
            46.43, 44.91, 41.20, 
            42.27, 43.85, 42.58, 
            40.59, 39.59, 36.44, 
            36.58, 43.21, 40.04, 
            43.30, 31.63, 41.62, 
            42.89, 43.03, 34.62, 
            46.51, 37.22, 41.74, 
            38.96, 37.96, 43.45, 
            35.74, 51.83, 34.93, 
            42.62, 38.50, 37.42, 
            35.83, 41.82, 30.99, 
            35.19, 48.88, 46.03, 
            39.39, 33.80, 37.70, 
            45.93, 41.10, 49.28, 
            37.21, 42.74, 33.16, 
            41.12, 50.00, 38.17, 
            46.60, 45.35, 49.78, 
            41.33, 36.05, 40.19, 
            40.34, 28.92, 30.05, 
            36.02, 36.96, 45.54, 
            40.20, 44.88, 43.46, 
            38.26, 40.44, 34.26, 
            40.33, 33.20, 43.74, 
            49.45, 42.24, 40.69, 
            34.77, 50.16, 42.68, 
            45.14, 45.09, 35.40, 
            36.05, 40.00, 40.89, 
            39.82, 37.37, 42.83, 
            43.37, 42.08, 34.03, 
            46.19, 33.16, 45.16, 
            36.71, 38.90, 42.42, 
            32.82, 44.10, 33.35, 
            34.20, 40.67, 36.95, 
            42.83, 42.08, 44.11, 
            40.01, 37.53, 37.78, 
            37.01, 40.43, 42.23, 
            30.48, 38.05, 42.32, 
            45.47, 35.22, 49.53, 
            40.42, 42.20, 45.66, 
            37.04, 36.05, 32.70, 
            38.24, 40.41, 34.19, 
            38.24, 46.50, 39.67, 
            41.87, 42.32, 42.79, 
            43.20, 39.89, 33.99, 
            42.13, 37.92, 36.16, 
            37.40, 44.43, 32.92, 
            48.68, 38.93, 38.48, 
            43.53, 40.24, 29.23, 
            43.87, 41.08, 36.74, 
            38.20, 34.53, 39.37, 
            38.96, 43.18, 43.13, 
            38.33, 30.31, 34.59, 
            42.51, 38.60, 35.53, 
            38.21, 36.93, 38.95, 
            36.22, 45.71, 36.46, 
            39.60, 32.22, 30.85, 
            50.00, 45.38, 36.14, 
            34.59, 44.88, 40.80, 
            39.16, 39.17, 46.33, 
            36.62, 41.01, 45.16, 
            38.13, 37.90, 47.41, 
            42.60, 37.25, 41.44, 
            45.73, 39.41, 40.20, 
            35.65, 35.93, 32.17, 
            37.50, 38.41, 38.81, 
            36.62, 45.13, 54.81, 
            25.04, 30.42, 34.40, 
            43.64, 50.07, 42.33, 
            35.57, 36.89, 43.05, 
            36.45, 40.31, 46.72, 
            39.60, 43.88, 47.33, 
            35.79, 50.65, 36.37, 
            44.31, 39.66, 38.99, 
            53.15, 34.43, 41.63, 
            42.01, 50.46, 40.95, 
            37.50, 43.78, 39.61, 
            41.06, 41.52, 40.98, 
            34.75, 40.46, 44.30, 
            40.22, 35.69, 45.78, 
            51.93, 34.77, 38.99, 
            32.27, 35.38, 43.63, 
            41.58, 38.29, 40.12, 
            40.08, 31.37, 36.36, 
            38.17, 41.91, 30.43, 
            40.50, 42.75, 37.53, 
            37.89, 48.52, 45.83, 
            36.58, 41.74, 49.49, 
            44.66, 34.87, 41.54, 
            42.39, 45.24, 45.00, 
            35.53, 45.08, 37.21, 
            36.15, 39.22, 45.56, 
            39.20, 43.10, 39.74, 
            38.71, 42.98, 44.35, 
            46.65, 43.13, 40.62, 
            36.92, 46.80, 51.72, 
            41.18, 35.46, 43.89, 
            46.81, 35.46, 40.36, 
            48.46, 41.24, 42.43, 
            36.48, 41.63, 40.27, 
            36.13, 40.60, 36.78, 
            38.36, 39.13, 44.34, 
            41.73, 47.99, 46.57, 
            45.16, 36.83, 32.58, 
            40.78, 40.04, 48.19, 
            39.10, 37.10, 46.00, 
            54.55, 30.64, 34.96, 
            39.62, 38.71, 50.25, 
            36.56, 39.73, 37.00, 
            37.61, 35.75, 34.81, 
            42.46, 46.23, 51.15, 
            38.56, 42.93, 49.20, 
            50.23, 47.21, 50.64, 
            43.10, 46.28, 33.94, 
            38.52, 35.16, 47.78, 
            45.58, 41.58, 45.18, 
            32.40, 34.94, 39.09, 
            38.12, 46.70, 33.39, 
            51.96, 31.74, 36.72, 
            37.43, 44.07, 37.63, 
            42.68, 40.98, 42.75, 
            41.51, 45.38, 41.07, 
            43.80, 46.07, 40.68, 
            44.06, 36.14, 35.97, 
            40.89, 42.20, 32.19, 
            31.84, 46.36, 37.59, 
            42.48, 43.86, 31.56, 
            38.54, 44.58, 38.40, 
            46.39, 39.95, 35.21, 
            30.95, 37.37, 40.71, 
            42.09, 42.65, 35.56, 
            30.68, 46.66, 39.31, 
            32.26, 42.19, 41.06, 
            36.23, 30.62, 38.14, 
            38.60, 37.41, 30.99, 
            36.33, 45.70, 43.07, 
            44.70, 40.19, 44.16, 
            30.91, 34.49, 40.54, 
            45.16, 40.58, 42.08, 
            37.52, 39.55, 43.83, 
            46.07, 36.09, 39.61, 
            32.78, 28.37, 39.56, 
            42.55, 47.59, 37.19, 
            36.87, 34.52, 36.79, 
            40.39, 38.50, 37.41, 
            41.87, 37.03, 39.63, 
            36.94, 40.08, 39.39, 
            26.72, 45.60, 34.68, 
            40.03, 34.59, 42.62, 
            37.33, 35.91, 32.14, 
            37.73, 50.10, 35.61, 
            43.75, 40.93, 40.30, 
            36.56, 36.56, 45.93, 
            34.98, 42.19, 49.34, 
            46.81, 33.98, 43.61, 
            46.13, 45.18, 47.29, 
            35.70, 35.25, 48.17, 
            35.65, 40.60, 40.83, 
            34.90, 41.98, 34.95, 
            40.67, 42.49, 47.93, 
            44.98, 39.91, 40.47, 
            37.14, 46.79, 46.49, 
            44.73, 42.96, 38.82, 
            43.70, 41.22, 37.88, 
            51.16, 47.96, 42.51, 
            31.27, 37.10, 36.08, 
            39.54, 36.90, 34.18, 
            33.42, 40.73, 39.73, 
            36.05, 40.82, 33.15, 
            45.66, 37.29, 46.42, 
            37.05, 29.44, 44.16, 
            40.17, 34.77, 38.09, 
            34.29, 37.17, 41.70, 
            40.27, 43.07, 40.26, 
            34.91, 40.66, 34.65, 
            35.59, 47.19, 44.48, 
            44.47, 43.34, 42.26, 
            41.55, 42.91, 35.10, 
            40.14, 43.23, 40.10, 
            45.86, 37.42, 34.31, 
            46.76, 40.03, 39.47, 
            40.83, 42.79, 35.75, 
            35.54, 48.18, 41.07, 
            39.99, 41.97, 38.33, 
            41.28, 30.75, 38.20, 
            32.34, 40.90, 38.22, 
            35.16, 44.10, 44.94, 
            38.62, 46.42, 36.92, 
            41.27, 41.28, 42.12, 
            43.49, 44.39, 39.91, 
            33.77, 48.58, 37.60, 
            36.00, 44.11, 32.36, 
            44.50, 42.83, 36.92, 
            33.39, 32.14, 48.65, 
            38.40, 34.42, 37.82, 
            42.35, 36.48, 39.97,
            ];
            
            for &num in nums.iter() {
                x_increment = t1_access.write().push_float(num, x_increment);
                

                if t1_access.read().get_length() == 4 && !(t2_access.read().vec.is_empty()) {
                    let t1_average;
                    let mut vec_len;
                    let t1_last_elem;

                    println!("Tier 1 {:?}", t1_access.read().get_y());

                    {
                    let mut t1_lock = t1_access.write();  
                    let vec_slice = &t1_lock.vec[0..t1_lock.vec.len() - 1]; 
                    t1_average = t1_lock.calculate_average(vec_slice);
                    vec_len = t1_lock.vec.len();
                    t1_lock.vec.drain(0..vec_len - 1);
                    t1_last_elem = t1_lock.vec[0];
                    }

                    {
                        let mut t2_write = initial_tier.write();
                        let length = t2_write.vec.len() - 1;
                        t2_write.vec[length] = t1_average;
                        t2_write.vec.push(t1_last_elem);
                    }

                    // println!("After drain {:?}", t1_access.read().get_y());
                    // println!("Average is {}", t1_average[1]);
                    println!("Tier 1 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\n", t1_access.read().get_y(), initial_tier.read().get_y(), t3_access.read().get_y());

                } else if t1_access.read().get_length() == 4 && t2_access.read().vec.is_empty() {
                    process_tier_empty_case(&t1_access, &t2_access);
                }


                if t2_access .read().get_length() == 4 { 
                    process_tier(&t2_access, &t3_access);
                    println!("Tier 2 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\n", t1_access.read().get_y(), initial_tier.read().get_y(), t3_access.read().get_y());
                }

                /*
                if t1_access.read().get_length() == 8 && t2_access.read().vec.is_empty() {
                    println!("Do an empty fill");

                    let mut vec_len;
                    let t1_average;
                    let t1_last_elem;
                    let t3_vec;
                    let t3_len;
                    {
                        let mut t1_lock = t1_access.write();
                        let vec_slice = &t1_lock.vec[0..t1_lock.vec.len() - 1];
                        t1_average = t1_lock.calculate_average(vec_slice);
                        vec_len = t1_lock.vec.len();
                        t1_lock.vec.drain(0..vec_len - 1);
                        t1_last_elem = t1_lock.vec[0];
                    }

                    {
                        let mut t3_lock = t3_access.write();
                        t3_vec = t3_lock.vec.clone();
                        t3_len = t3_lock.vec.len();
                        t3_lock.vec[t3_len-1] = t1_average;
                    }

                    {
                        let mut t2_write = t2_access.write();
                        //t2_write.vec.push(t3_vec[t3_len - 2]);
                        t2_write.vec.push(t1_average);
                        t2_write.vec.push(t1_last_elem);
                    }
                    println!("Tier 1 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\n", t1_access.read().get_y(), t2_access.read().get_y(), t3_access.read().get_y());

                } else if t1_access.read().get_length() == 8 {
                    //println!("TIER 1 start {}", t1_count);
                    //t1_count +=1;
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

                    println!("Tier 1 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\n", t1_access.read().get_y(), t2_access.read().get_y(), t3_access.read().get_y());
                }

                /*Keep in mind length first element of t2 is previous element of t3, thus subtract 1 from condition. I.E if merging when length 7, means every six bins added merge
                When plotting it appears as every 5 bins then on the sixth bin the merge occurs*/
                if t2_access.read().vec.len() == 8 {
                    //println!("TIER 2 start {}", t2_count);
                    //t2_count +=1;
                    let mut vec_len;
                    let t2_average;
                    let t1_elem = t1_access.read().vec[0];

                    {
                        let mut t2_lock = t2_access.write();
                        let vec_slice = &t2_lock.vec[1..t2_lock.vec.len()-1];
                        t2_average = t2_lock.calculate_average(vec_slice);
                        vec_len = t2_lock.vec.len();
                        t2_lock.vec.drain(0..vec_len);
                    }
                    //println!("Avg: {}", t2_average[1]);

                    {
                        let mut t3_write = t3_access.write();
                        t3_write.vec.push(t2_average);
                        t3_write.vec.push(t1_elem)
                    }
                    println!("Tier 2 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\n", t1_access.read().get_y(), t2_access.read().get_y(), t3_access.read().get_y());
                }
             

                if t3_access.read().vec.len() == 12 {
                    println!("\nTIER 3 start {}", t3_count);
                    t3_count +=1;
                    let mut vec_len;
                    let t3_average;

                    {
                        let t3_lock = t3_access.read();
                        let vec_slice = &t3_lock.vec[1..t3_lock.vec.len()-1];
                        t3_average = t3_lock.calculate_average(vec_slice);
                        vec_len = t3_lock.vec.len();
                    }
                    //println!("Avg: {}", t3_average[1]);

                    {
                        let mut t4_write = t4_access.write();
                        t4_write.vec.push(t3_average);
                    }

                    {
                        let mut t3_write = t3_access.write();
                        t3_write.vec[0] = t3_average;
                        
                        t3_write.vec.drain(1..vec_len -1 );
                    }


                    println!("Tier 3 drain\nt1: {:?}\nt2: {:?}\nt3: {:?}\nt4: {:?}\n", t1_access.read().get_y(), t2_access.read().get_y(), t3_access.read().get_y(), t4_access.read().get_y());
                }   */             

                thread::sleep(Duration::from_millis(500));
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

fn process_tier(current_tier: &Arc<RwLock<Tier>>, lower_tier: &Arc<RwLock<Tier>>) {
    //println!("Need to aggregate this {:?}", current_tier.read().get_y());

    let mut vec_len: usize;
    let current_average;
    let vec_len;
    let last_elem;
    let lower_legnth;

    { 
        let mut current_lock = current_tier.write();
        vec_len = current_lock.vec.len();
        let vec_slice = &current_lock.vec[0..vec_len-1];
        current_average = current_lock.calculate_average(vec_slice);

        last_elem = current_lock.vec[vec_len-1];
        current_lock.vec.drain(0..vec_len);

        // println!("Average {}", current_average[1]);
        // println!("Tier {:?}", current_lock.get_y());
        // println!("Last elem {}", last_elem[1]);
    }

    {
        let mut current_write = lower_tier.write();
        lower_legnth = current_write.vec.len();
        current_write.vec[lower_legnth-1] = current_average;
        current_write.vec.push(last_elem);
    }
}

fn process_tier_empty_case(current_tier: &Arc<RwLock<Tier>>, lower_tier: &Arc<RwLock<Tier>>) {
    println!("Doing the empty drain Current tier {:?}", current_tier.read().get_y());

    /* Have to fix this here, give up for now!!!
    let current_average;
    let vec_len;
    let last_elem;
    //let lower_legnth;

    { 
        let mut current_lock = current_tier.write();
        vec_len = current_lock.vec.len();
        let vec_slice = &current_lock.vec[0..vec_len-1];
        current_average = current_lock.calculate_average(vec_slice);

        last_elem = current_lock.vec[vec_len-1];
        current_lock.vec.drain(0..vec_len);

        println!("Average {}", current_average[1]);
        println!("Tier {:?}", current_lock.get_y());
        println!("Last elem {}", last_elem[1]);
    }*/

    // {
    //     let mut current_write = lower_tier.write();
    //     //lower_legnth = current_write.vec.len();
    //     current_write.vec.push(current_average);
    //     current_write.vec.push(last_elem);
    // }
} 

impl eframe::App for MyApp {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
    egui::CentralPanel::default().show(ctx, |ui| { 
        ctx.set_visuals(Visuals::light());
        
        let t1_line = Line::new( self.t1.read().get_points()).width(2.0).color(Color32::RED);
        let t2_line = Line::new(self.t2.read().get_points()).width(2.0).color(Color32::BLUE);
        let t3_line = Line::new(self.t3.read().get_points()).width(2.0).color(Color32::GREEN);
        //let t4_line = Line::new(self.t4.read().get_points()).width(2.0);

        let plot = Plot::new("plot")
        .min_size(Vec2::new(800.0, 600.0));

        plot.show(ui, |plot_ui| {
            plot_ui.line(t1_line);
            plot_ui.line(t2_line);
            plot_ui.line(t3_line);
            //plot_ui.line(t4_line);
        });
    });
    ctx.request_repaint();
    }
}

