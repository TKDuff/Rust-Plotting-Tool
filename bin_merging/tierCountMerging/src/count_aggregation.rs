use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};
use crate::aggregation_strategy::AggregationStrategy;
use crate::bin::Bin;

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


        let x = Data::new(x_vec.clone());   
        let y = Data::new(y_vec.clone());

        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        let y_sum: f64 = y_vec.iter().sum();
        

        self.x_stats.push(Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len() });
        self.y_stats.push(Bin {mean: y_mean, sum: y.iter().sum() , min: y.min(), max: y.max(), count: y.len() });

        println!("The sum is: {} The length is: {}, The y mean is {}, The x mean is {}", y_sum, y.len(), y_mean, x_mean);

        (x_mean, y_mean, x.len())
    }

    fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }
}