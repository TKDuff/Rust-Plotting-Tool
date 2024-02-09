use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};
use crate::aggregation_strategy::AggregationStrategy;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Clone, Default)] //allow deriving clones
pub struct Bin {
    pub mean: f64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub count: usize,
    pub timestamp: u64,
}

impl Bin {
    fn print(&self) {
        println!("Mean {}\nSum {}\n Min {}\nMax {}\nCount {}",self.mean, self.sum, self.min, self.max, self.count);
    }

}

pub struct CountAggregateData {
    pub x_stats: Vec<Bin>,
    pub y_stats: Vec<Bin>,
}

impl CountAggregateData {
    pub fn new() -> Self {
        Self { 
            x_stats: Vec::new(),
            y_stats: Vec::new(),
        }
    }
}

impl AggregationStrategy for CountAggregateData {

    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (f64, f64, usize) {
        //println!("\nAggregating this chunk {:?}", chunk);
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().map(|&[x, y]| (x, y)).unzip();


        let mut x = Data::new(x_vec.clone());   
        let mut y = Data::new(y_vec.clone());

        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        let y_sum: f64 = y_vec.iter().sum();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        

        self.x_stats.push(Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len(), timestamp: timestamp });
        self.y_stats.push(Bin {mean: y_mean, sum: y.iter().sum() , min: y.min(), max: y.max(), count: y.len(), timestamp: timestamp });

        println!("sum:{} length: {},y mean: {},x mean: {} timestamp: {}", y_sum, y.len(), y_mean, x_mean, timestamp);

        (x_mean, y_mean, x.len())
    }

    fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }

    fn categorise_recent_bins(&self, seconds_interval: u64) {
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    
    println!("The current timestamp is: {}", current_timestamp);

    let temp = self.x_stats.iter()
        .filter(|bin| current_timestamp - bin.timestamp <= seconds_interval)
        .cloned()
        .collect::<Vec<Bin>>();

    println!("Points collected to far {}\n", temp.len());


    }
}