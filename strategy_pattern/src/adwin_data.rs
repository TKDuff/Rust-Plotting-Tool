use crate::data_strategy::DataStrategy;

pub struct AdwinRawData {
    pub delta: f64,
    pub points: Vec<[f64;2]>,
}

impl AdwinRawData {
    pub fn new() -> Self {
        Self { 
            delta: 0.000000000000000000000000000001,
            points: vec![[0.0, 0.0]]
        }
    }

    // Private non-interface helper methods to compute the means of two sub-windows
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
}

impl DataStrategy for AdwinRawData {
    fn append_str(&mut self, line:String) {
        let values_result: Result<Vec<f64>, _> = line.split(' ')
        .map(|s| s.trim().parse::<f64>())
        .collect();

        match values_result {
            Ok(values) => {
                self.append_point(values[0], values[1]);
            }
            Err(err) => {
                println!("Error parsing values: {:?}", err);
            }
        }
    }

    fn get_raw_data(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }

    fn append_point(&mut self, x_value: f64, y_value: f64) {
        self.points.push([x_value, y_value]);
    }

    fn requires_external_trigger(&self) -> bool {
        false
    }

    fn get_values(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }

    fn remove_chunk(&mut self, count:usize, point_means: (f64, f64)) {
        self.points[0] = [point_means.0, point_means.1];
        self.points.drain(1..count+1);
    }

    fn check_cut(&self) -> Option<usize> {
        let window_y_values: &[f64] = &self.points.iter().skip(1).map(|&[_, y]| y).collect::<Vec<f64>>()[..];
        for i in 1..window_y_values.len() {
            let (mean1, mean2, fC, sC) = self.compute_means(i, window_y_values);
            let n1 = i as f64;
            let n2 = window_y_values.len() as f64 - n1;
            let epsilon = self.compute_epsilon(n1, n2);
            if (mean1 - mean2).abs() > epsilon {
                println!("fC{:?}\nsC{:?}", fC, sC);
                println!("The cut index is {}", i);
                return Some(i); // Return Some(index) where the cut should occur
            }
        }
        None // Return None if no cut is needed
    }

    fn get_chunk(&self, count:usize) -> Vec<[f64;2]> {
        self.points.clone()[..count + 1].to_vec()
    }
}