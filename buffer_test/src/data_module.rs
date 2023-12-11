use std::usize;

use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};

pub struct StdinData {
    pub points: Vec<[f64;2]>,
}

impl StdinData {
    pub fn new() -> Self {
        Self { points: Vec::default(),}
    }

    pub fn append_points(&mut self, points: [f64; 2]) {
        self.points.push([points[0], points[1]]);
        }

    /*Takes in line string from standard input, converts two string numbers to float, appends them to the vector of points to plot*/
    pub fn append_str(&mut self, s:&str) {
        let parts: Vec<&str> = s.split_whitespace().collect();
        self.append_points([parts[0].parse::<f64>().unwrap(), parts[1].parse::<f64>().unwrap()]);
    }

    pub fn get_length(&self) -> usize {
        self.points.len()
    }

    pub fn get_chunk(&self, count:usize) -> Vec<[f64;2]> {
        self.points[0..count].to_vec()
    }

    /*So use into_iter if you want to consume the entire collection, and use drain if you only want to consume part of the collection or if you want to reuse the emptied collection later. */
    pub fn remove_chunk(&mut self, count:usize, point_means: (f64, f64)) {
        self.points[0] = [point_means.0, point_means.1];
        self.points.drain(1..count);
    }

    pub fn get_values(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }
}


pub struct DownsampledData {
    pub x_stats: Vec<[f64;6]>,
    pub y_stats: Vec<[f64;6]>,
}
impl DownsampledData {
    pub fn new() -> Self {
        Self { 
            x_stats: vec![[0.0; 6]],//Vec::default(),
            y_stats: vec![[0.0; 6]],//Vec::default(),
        }
    }

    pub fn append_statistics(&mut self, chunk: Vec<[f64;2]>) -> (f64, f64) {
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().map(|&[x, y]| (x, y)).unzip(); //refactor, calculate mean while iterating over             

        let mut x = Data::new(x_vec.clone());   
        let mut y = Data::new(y_vec.clone());

        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        let x_sum_of_squares: f64 = x_vec.iter().map(|&xi| (xi - x_mean).powi(2)).sum();
        let y_sum_of_squares: f64 = y_vec.iter().map(|&yi| (yi - y_mean).powi(2)).sum();





        self.x_stats.push([x_mean, x_sum_of_squares, x.upper_quartile(), x.median(), x.min(), x.max()]);
        self.y_stats.push([y_mean, y_sum_of_squares, y.upper_quartile(), y.median(), y.min(), y.max()]); 

        (x_mean, y_mean) //returned as replace aggregated chunk with with the average value, fills gap between two plots
    }

    pub fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x[0], y[0]]) // Assuming index 5 is the mean
            .collect()
    }
}