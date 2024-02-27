use std::usize;
extern crate lttb;

use lttb::{DataPoint,lttb};
use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};

pub struct StdinData {
    pub points: Vec<[f64;2]>,
}

impl StdinData {
    pub fn new() -> Self {
        let points = vec![
            [0.0, 0.0],
            [1.0, 1.0],
            [2.0, 1.0],
            [3.0, 1.0],
            [4.0, 1.0],
            [5.0, 1.0],
            // Add more points as needed
        ];
        
        Self { points }
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
    pub x_stats: Vec<[f64;2]>,
    pub y_stats: Vec<[f64;2]>,
}
impl DownsampledData {
    pub fn new() -> Self {
        Self { 
            x_stats: vec![[0.0; 2]],//Vec::default(),
            y_stats: vec![[0.0; 2]],//Vec::default(),
        }
    }

    pub fn append_statistics(&mut self, chunk: Vec<[f64;2]>, point_count:usize) -> (f64, f64) {
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().map(|&[x, y]| (x, y)).unzip(); //refactor, calculate mean while iterating over             

        let mut x = Data::new(x_vec.clone());   
        let mut y = Data::new(y_vec.clone());

        /*Look into using moving average */
        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        self.x_stats.push([x_mean, point_count as f64]);
        self.y_stats.push([y_mean, point_count as f64]); 

        (x_mean, y_mean) //returned as replace aggregated chunk with with the average value, fills gap between two plots
    }

    

    pub fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x[0], y[0]]) // Assuming index 5 is the mean
            .collect()
    }

    pub fn combineBins(&mut self, lttb_points:usize) {
        let mut raw = vec!();
        let combined_vec: Vec<[f64; 2]> = self.x_stats.iter().zip(self.y_stats.iter()).take(lttb_points - 1)
                                                        .map(|(x, y)| [x[0], y[0]]) // Change indices if needed
                                                        .collect();
        
        raw = combined_vec.into_iter()
                               .map(|[x, y]| DataPoint::new(x, y)) // Corrected destructuring for an array
                               .collect();

        let length = raw.len();
        let downsampled = lttb(raw, length/2);
        self.x_stats = downsampled.iter().map(|dp| [dp.x, 0.0]).collect(); //[dp.x, dp.y] - array of 2 f64 while (dp.x, dp.y) - tuple of 2 f64
        self.y_stats = downsampled.iter().map(|dp| [dp.y, 0.0]).collect();

        /*
        println!("Post-downsample {:?}\nLength {}\n", downsampled, downsampled.len());*/
    }
    

}