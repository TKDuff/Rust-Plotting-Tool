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
    pub fn remove_chunk(&mut self, count:usize) {
        //println!("\n\n{:?} {}\n", self.points, self.points.len());
        self.points.drain(0..count);
        //println!("{:?} {}\n\n", self.points, self.points.len());
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

    pub fn append_statistics(&mut self, chunk: Vec<[f64;2]>, point_count:usize, length:usize) {

        //split chunk into two vectors, exclude the last points,these are the points in the next chunk
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter() //create iterator
                                                .take(chunk.len().saturating_sub(1)) // all but the last element of iterator
                                                .map(|&[x, y]| (x, y)) //map each pair [x,y] to tuple (x,y)
                                                .unzip(); //seperate tuple into two seperate vectors                       

        let mut x = Data::new(x_vec);   
        let mut y = Data::new(y_vec);

        //replace the previous next chunk beginning point with the statistic values for each x,y vector
        self.x_stats[length] = [x.lower_quartile(), x.upper_quartile(), x.median(), x.min(), x.max(), x.mean().unwrap()];
        self.y_stats[length] = [y.lower_quartile(), y.upper_quartile(), y.median(), y.min(), y.max(), y.mean().unwrap()];

        //push the beginning point of the next chunk
        self.x_stats.push([0.0, 0.0, 0.0, 0.0, 0.0, chunk[10][0]]);
        self.y_stats.push([0.0, 0.0, 0.0, 0.0, 0.0, chunk[10][1]]);

        /*
        Fill historic and raw data gap as follows
        -exist x_stats & y_stats vectors, initialised with single point 
        -take in vector (chunk), the size is the number of points to downsample plus 1, so if downsample 10 points, chunk size is 11, [1..11]
        -split 2d vector into respective x,y vectors, each vector length 10, [1..10], excluding the last point
        -aggregate each respective vector points into summary stats (x,y)
        -Replace the x_stats, y_stat current point at the end with the respective summary statistics
        -Push the last point from the taken in chunk (excluded from aggregation) to the repsective vector, so point 11 in this case

        This ensures the last point in hisotric data plot is the first point in the next chunk to be downsample
         */
    }

    pub fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x[5], y[5]]) // Assuming index 5 is the mean
            .collect()
    }
}