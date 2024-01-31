use std::usize;

use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};

pub struct StdinData {
    pub points: Vec<[f64;2]>,
}

impl StdinData {
    pub fn new() -> Self {
        Self { points: vec![[0.0, 0.0]],}
    }

    pub fn append_points(&mut self, points: [f64; 2]) {
        self.points.push([points[0], points[1]]);
        }

    pub fn append_str(&mut self, line:String) {
        let values_result: Result<Vec<f64>, _> = line.split(' ')
        .map(|s| s.trim().parse::<f64>())
        .collect();

        match values_result {
            Ok(values) => {
                println!("{}", values[1]);
                self.points.push([values[0], values[1]]);
            }
            Err(err) => {
                println!("Error parsing values: {:?}", err);
            }
        }
    }

    pub fn get_length(&self) -> usize {
        self.points.len()
    }

    pub fn get_chunk(&self, count:usize) -> Vec<[f64;2]> {
        println!("Here {} {}", count, self.points.len());
        self.points[..count].to_vec()
    }

    /*So use into_iter if you want to consume the entire collection, and use drain if you only want to consume part of the collection or if you want to reuse the emptied collection later. */
    pub fn remove_chunk(&mut self, count:usize, point_means: (f64, f64)) {
        self.points[0] = [point_means.0, point_means.1];
        self.points.drain(1..count+1);
    }


    pub fn get_values(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }
}



#[derive(Clone, Default)] //allow deriving clones
pub struct Bin {
    pub mean: f64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub count: usize,
}
impl Bin {
    fn print(&self) {
        println!("Mean {}\nSum {}\n Min {}\nMax {}\nCount {}",self.mean, self.sum, self.min, self.max, self.count);
    }
}

pub struct AggregateData {
    pub x_stats: Vec<Bin>,
    pub y_stats: Vec<Bin>,
}

impl AggregateData {
    pub fn new() -> Self {
        Self { 
            x_stats: Vec::new(),
            y_stats: Vec::new(),
        }
    }


    pub fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (f64, f64, usize) {
           
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().skip(1).map(|&[x, y]| (x, y)).unzip();


        let mut x = Data::new(x_vec.clone());   
        let mut y = Data::new(y_vec.clone());

        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        let y_sum: f64 = y_vec.iter().sum();
        

        self.x_stats.push(Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len() });
        self.y_stats.push(Bin {mean: y_mean, sum: y.iter().sum() , min: y.min(), max: y.max(), count: y.len() });

        println!("\nThe sum is: {} The lenght is: {}, The y mean is {}", y_sum, y.len(), y_mean);

        (x_mean, y_mean, x.len())
    }

    pub fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }

}