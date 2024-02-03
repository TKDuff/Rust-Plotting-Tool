// Import necessary standard library components
use std::collections::VecDeque;
use std::f64;

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

    // Method to add a new data point to the ADWIN window
    pub fn add(&mut self, x_value: f64, y_value: f64) {
        self.window.push([x_value, y_value]); // Append the value to the end of the window
    
        // Create a slice of y-values
        // let y_values: Vec<f64> = self.window.iter().map(|&[_, y]| y).collect();
        // let y_values_slice = &y_values[..];

        let y_values_slice: &[f64] = &self.window.iter().skip(1).map(|&[_, y]| y).collect::<Vec<f64>>()[..];

        println!("{:?}", y_values_slice);

        
        if let (Some(cut_index), mean_y) = self.check_cut(y_values_slice) {

            // Calculate the mean x-value for the cut window
            let mean_x = self.window.iter().take(cut_index).map(|&[x, _]| x).sum::<f64>() / cut_index as f64;
            self.cut_window(cut_index, mean_x, mean_y);
        }
    }

    // Method to check if a cut is needed in the window
    // Is the condition of the async method
    fn check_cut(&self, window_y_values: &[f64]) -> (Option<usize>, f64) {
        for i in 1..window_y_values.len() {
            let (mean1, mean2) = self.compute_means(i, window_y_values);
            let n1 = i as f64;
            let n2 = window_y_values.len() as f64 - n1;
            let epsilon = self.compute_epsilon(n1, n2);
            if (mean1 - mean2).abs() > epsilon {
                return (Some(i), mean1); // Return Some(index) where the cut should occur
            }
        }
        (None, 0.0) // Return None if no cut is needed
    }

    //remove chunk
    fn cut_window(&mut self, cut_index: usize, x_mean: f64, y_mean: f64) {
        self.window[0] = [x_mean, y_mean];
        self.window.drain(1..cut_index+1);
        //self.window.drain(..cut_index); // Drain the elements up to the cut index
        println!("{} {}",x_mean, y_mean);
        self.aggregate_points.push([x_mean, y_mean]);
    }
    
    
    // Helper method to compute the means of two sub-windows
    // append_chunk_aggregate_statistics
    fn compute_means(&self, split_index: usize, window_y_values: &[f64]) -> (f64, f64) {
        let sum1: f64 = window_y_values.iter().take(split_index).sum();
        let mean1 = sum1 / split_index as f64;
    
        let sum2: f64 = window_y_values.iter().skip(split_index).sum();
        let mean2 = sum2 / (window_y_values.len() - split_index) as f64;
        (mean1, mean2)
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

}
