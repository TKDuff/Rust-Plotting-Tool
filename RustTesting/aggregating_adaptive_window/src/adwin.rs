// Import necessary standard library components
use std::collections::VecDeque;
use std::f64;

// Define the ADWIN struct
pub struct ADWIN {
    delta: f64,          // The delta parameter, determining sensitivity to change
    window: Vec<f64>, // The window of data points, implemented as a double-ended queue
    raw_points: Vec<[f64;2]>,
    aggregate_points: Vec<[f64;2]>

}

// Implementation block for ADWIN
impl ADWIN {
    // Constructor for creating a new ADWIN instance
    pub fn new(delta: f64) -> ADWIN {
        ADWIN {
            delta, // Set the delta value
            window: Vec::new(), // Initialize an empty VecDeque for the window
            raw_points: Vec::default(),
            aggregate_points: Vec::default(),
        }
    }

    pub fn append_str(&mut self, line:String) {
        let values_result: Result<Vec<f64>, _> = line.split(',')
        .map(|s| s.trim().parse::<f64>())
        .collect();

        match values_result {
            Ok(values) => {
                self.raw_points.push([values[0], values[1]]);
                self.add(values[0], values[1]);
            }
            Err(err) => {
                println!("Error parsing values: {:?}", err);
            }
        }


    }

    // Method to add a new data point to the ADWIN window
    pub fn add(&mut self, x_value: f64, y_value: f64) {
        self.window.push(y_value); // Append the value to the end of the window

        if let (Some(cut_index), mean) = self.check_cut() {
            self.cut_window(cut_index, mean, x_value);
        } 
    }

    // Method to potentially cut the window
    fn cut_window(&mut self, cut_index: usize, mean: f64, x_value: f64) {
        self.window.drain(..cut_index); // Drain the elements up to the cut index
        println!("Now pushing {} {} to the aggregate_points", x_value, mean);
        self.aggregate_points.push([x_value, mean]);
    }

    // Method to check if a cut is needed in the window
    fn check_cut(&self) -> (Option<usize>, f64) {
        for i in 1..self.window.len() {
            let (mean1, mean2) = self.compute_means(i);
            let n1 = i as f64;
            let n2 = self.window.len() as f64 - n1;
            let epsilon = self.compute_epsilon(n1, n2);
            if (mean1 - mean2).abs() > epsilon {
                return (Some(i), mean1); // Return Some(index) where the cut should occur
            }
        }
        (None, 0.0) // Return None if no cut is needed
    }

    // Helper method to compute the means of two sub-windows
    fn compute_means(&self, split_index: usize) -> (f64, f64) {
        // Compute sum and mean of elements before the split index
        let sum1: f64 = self.window.iter().take(split_index).sum();
        let mean1 = sum1 / split_index as f64;

        // Compute sum and mean of elements after the split index
        let sum2: f64 = self.window.iter().skip(split_index).sum();
        let mean2 = sum2 / (self.window.len() - split_index) as f64;
        // Return the two means
        (mean1, mean2)
    }

    fn compute_epsilon(&self, n1: f64, n2: f64) -> f64 {
        (1.0 / (2.0 * n1) * (4.0 / self.delta).ln()).sqrt() + 
        (1.0 / (2.0 * n2) * (4.0 / self.delta).ln()).sqrt()
    }

    pub fn get_aggregate_points(&self) -> Vec<[f64; 2]> {
        self.aggregate_points.clone().into_iter().collect()
    }

    pub fn get_raw_points(&self) -> Vec<[f64; 2]> {
        self.raw_points.clone().into_iter().collect()
    }
}
