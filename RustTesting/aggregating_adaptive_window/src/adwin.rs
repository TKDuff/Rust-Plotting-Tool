// Import necessary standard library components
use std::collections::VecDeque;
use std::f64;
use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};

// Define the ADWIN struct
pub struct ADWIN {
    delta: f64,          // The delta parameter, determining sensitivity to change
    window: Vec<[f64;2]>, // The window of data points, implemented as a double-ended queue
    aggregate_points: Vec<[f64;2]>

}

// Implementation block for ADWIN
impl ADWIN {
    // Constructor for creating a new ADWIN instance
    pub fn new(delta: f64) -> ADWIN {
        ADWIN {
            delta, // Set the delta value
            window: vec![[0.0, 0.0]], // Initialize an empty VecDeque for the window
            aggregate_points: Vec::default(),
        }
    }

    pub fn append_str(&mut self, line:String) {
        let values_result: Result<Vec<f64>, _> = line.split(',')
        .map(|s| s.trim().parse::<f64>())
        .collect();
        match values_result {
            Ok(values) => {
                self.add(values[0], values[1]);
            }
            Err(err) => {
                println!("Error parsing values: {:?}", err);
            }
        }
    }

    /*
    The first element in the sliding adwin windows the aggreated mean of the previous N1 window. Thus, when the 'check_cut()' function is called on a window [a..i] the element at a is excluded
    so the means of [b..g] is computed. When the cut index is found, the window is cut into two windows, N1 and N2.Say the cut is at 'e', N1 = [b,c,d,e] and N2 = [f,g,h,i]
    N1 is the sub-window up to the cut point, the mean is computed for the vector, call is mean_N1
    N2 is the new sliding window, to which points are added and the algorithm continous

    They key part is the first element of N2 is mean_N1. So for the original window, [a..i], upon cutting N2 = [mean_N1, f, g, h, i]
    When a new point is added to N2, say j, the algorithm checks the cut and computes means for [f..j], excluding the first element, mean_N1. 
    This is done for the egui_plot, in order to maintain consistency
     */
    // Method to add a new data point to the ADWIN window
    pub fn add(&mut self, x_value: f64, y_value: f64) {
        self.window.push([x_value, y_value]); // Append the value to the end of the window

        
        //vector window y elements excluding the first element, since its the previous N1 mean. 
        let y_values_slice_t: &[f64] = &self.window.iter().skip(1).map(|&[_, y]| y).collect::<Vec<f64>>()[..];

        // cut_index is indexed in relation to window as a whole, not the window excluding the first element
        if let (Some(cut_index), mean_y) = self.check_cut(y_values_slice_t) {
            println!("The current window is {:?}", self.window);
            self.append_chunk_aggregate_statistics(self.window[..cut_index+1].to_vec());
            let mean_x = self.window.iter().skip(1).take(cut_index).map(|&[x, _]| x).sum::<f64>() / cut_index as f64;
            self.cut_window(cut_index, mean_x, mean_y);
            println!("After cutting {:?}\n\n", self.window);
        }
    }

    // Method to check if a cut is needed in the window
    // Is the condition of the async method
    fn check_cut(&self, window_y_values: &[f64]) -> (Option<usize>, f64) {
        for i in 1..window_y_values.len() {
            let (mean1, mean2, fC, sC) = self.compute_means(i, window_y_values);
            let n1 = i as f64;
            let n2 = window_y_values.len() as f64 - n1;
            let epsilon = self.compute_epsilon(n1, n2);
            if (mean1 - mean2).abs() > epsilon {
                println!("fC{:?}\nsC{:?}", fC, sC);
                println!("The cut index is {}", i);
                return (Some(i), mean1); // Return Some(index) where the cut should occur
            }
        }
        (None, 0.0) // Return None if no cut is needed
    }

    //remove chunk
    fn cut_window(&mut self, cut_index: usize, x_mean: f64, y_mean: f64) {
        self.window[0] = [x_mean, y_mean];
        self.window.drain(1..cut_index+1);  // Drain the elements up to the cut index, increment it since cut_index is indexed based on full window, not excluding the first element
    }
    

    // Helper method to compute the means of two sub-windows
    // append_chunk_aggregate_statistics
    fn compute_means(&self, split_index: usize, window_y_values: &[f64]) -> (f64, f64, Vec<f64>, Vec<f64>) {
        let sum1: f64 = window_y_values.iter().take(split_index).sum();
        let mean1 = sum1 / split_index as f64;
    
        let sum2: f64 = window_y_values.iter().skip(split_index).sum();
        let mean2 = sum2 / (window_y_values.len() - split_index) as f64;

        let first_chunk: Vec<f64> = window_y_values.iter().take(split_index).cloned().collect();
        let second_chunk: Vec<f64> = window_y_values.iter().skip(split_index).cloned().collect();
        (mean1, mean2, first_chunk, second_chunk)
    }

    fn compute_epsilon(&self, n1: f64, n2: f64) -> f64 {
        (1.0 / (2.0 * n1) * (4.0 / self.delta).ln()).sqrt() + 
        (1.0 / (2.0 * n2) * (4.0 / self.delta).ln()).sqrt()
    }

    //get_means
    pub fn get_aggregate_points(&self) -> Vec<[f64; 2]> {
        self.aggregate_points.clone().into_iter().collect()
    }
    //get_values
    pub fn get_window_points(&self) -> Vec<[f64;2]> {
        self.window.clone().into_iter().collect()
    }

    pub fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) /*-> (f64, f64, usize)*/ {
           
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().skip(1).map(|&[x, y]| (x, y)).unzip();


        let mut x = Data::new(x_vec.clone());   
        let mut y = Data::new(y_vec.clone());

        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        let y_sum: f64 = y_vec.iter().sum();

        self.aggregate_points.push([x_mean, y_mean]);
        println!("\nx_Vec{:?}\ny_vec{:?}", x_vec, y_vec);
        println!("Method x mean is {} and y mean is {}\n", x_mean, y_mean);

        //(x_mean, y_mean, x.len())
    }

}