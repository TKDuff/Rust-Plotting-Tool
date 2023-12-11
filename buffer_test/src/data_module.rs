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

        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        /*
        let x_variance = x.variance().unwrap();
        let y_variance = y.variance().unwrap();

        let x_sum_of_squares: f64 = x_vec.iter().map(|&xi| (xi - x_mean).powi(2)).sum();
        let y_sum_of_squares: f64 = y_vec.iter().map(|&yi| (yi - y_mean).powi(2)).sum();


        self.x_stats.push([x_mean, x_sum_of_squares, point_count as f64, x_variance, x.min(), x.max()]);
        self.y_stats.push([y_mean, y_sum_of_squares, point_count as f64, y_variance, y.min(), y.max()]); */

        self.x_stats.push([x_mean, point_count as f64]);
        self.y_stats.push([y_mean, point_count as f64]); 

        (x_mean, y_mean) //returned as replace aggregated chunk with with the average value, fills gap between two plots
    }

    

    pub fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x[0], y[0]]) // Assuming index 5 is the mean
            .collect()
    }

    pub fn combineBins(&mut self) {
        let stat_bin_length = self.x_stats.len();
        let last_instances = stat_bin_length - 4;

        //println!("\nLength: {}\nLast Instance Index: {}", stat_bin_length, last_instances);
        let x_last_three = self.x_stats[last_instances..stat_bin_length-1].to_vec();
        let y_last_three = self.y_stats[last_instances..stat_bin_length-1].to_vec();
        
        let combined_x = self.get_combined_stats(&x_last_three);
        let combined_y = self.get_combined_stats(&y_last_three);



        //println!("Last three: {:?}\nTo be inserted: {:?}\nFull: {:?}", x_last_three, combined_x, self.x_stats);
        self.x_stats[last_instances-1] = combined_x;
        self.y_stats[last_instances-1] = combined_y;
        self.x_stats.drain(last_instances..stat_bin_length-1);
        self.y_stats.drain(last_instances..stat_bin_length-1);

        //println!("With drain{:?}", self.x_stats)





    }

    pub fn get_combined_stats(&self, stats: &Vec<[f64; 2]>) -> [f64; 2] {
        let total_count: f64 = stats.iter().map(|x| x[1]).sum();
        
        let combined_mean = stats.iter().map(|x| x[0] * x[1]).sum::<f64>() / total_count;

        //println!("Combined:\nTotal count: {}\nMean: {}", total_count, combined_mean);
        [combined_mean, total_count]
    }
}