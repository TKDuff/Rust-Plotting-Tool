use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};
use crate::aggregation_strategy::AggregationStrategy;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::bin::Bin;


pub struct CountAggregateData {
    pub x_stats: Vec<Bin>,
    pub y_stats: Vec<Bin>,
    pub seconds_length: usize,
}

impl CountAggregateData {
    pub fn new() -> Self {
        Self { 
            x_stats: Vec::new(),
            y_stats: Vec::new(),
            seconds_length: 0,
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
        let x_sum: f64 = x_vec.iter().sum();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        

        self.x_stats.push(Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len(), timestamp: timestamp });
        self.y_stats.push(Bin {mean: y_mean, sum: y.iter().sum() , min: y.min(), max: y.max(), count: y.len(), timestamp: timestamp });

        //println!("sum:{} length: {},y mean: {},x mean: {} timestamp: {}", y_sum, y.len(), y_mean, x_mean, timestamp);
        println!("sum:{} length: {},y mean: {},x mean: {} timestamp: {}", x_sum, x.len(), x_mean, x_mean, timestamp);

        (x_mean, y_mean, x.len())
    }

    fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }

    /*
    fn categorise_recent_bins(& mut self, seconds_interval: u128) {

        let length: usize = self.x_stats.len();

        println!("The chunk length is :{}\n", (length - self.seconds_length));
        self.seconds_length = length;

    }*/

    fn categorise_recent_bins(& mut self, seconds_interval: u128) {

        let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    
    //println!("The current timestamp is: {}", current_timestamp);

    let temp = self.x_stats.iter()
        .filter(|bin| current_timestamp - bin.timestamp <= seconds_interval)
        .cloned()
        .collect::<Vec<Bin>>();

    println!("Points collected to far {}", temp.len());
    for bin in temp {
        print!("{}, ", bin.get_timestamp());
    }
    println!("\n");

    }
    

    

    fn merge_vector_bins(&self, bins: &[Bin], y: usize) -> Vec<Bin> {
        let mut tempBin: Vec<Bin> = Vec::new();

        
        /*
        bins.chunks(y).for_each(|chunk| {
            // Calculate the sum and count for the current chunk
            let chunk_count: usize = chunk.iter().map(|bin| bin.count).sum();
            let chunk_sum: f64 = chunk.iter().map(|bin| bin.sum).sum();
            let chunk_min = chunk.iter().map(|bin| bin.min).fold(f64::INFINITY, f64::min);
            let chunk_max = chunk.iter().map(|bin| bin.max).fold(f64::NEG_INFINITY, f64::max);
            let chunk_mean = chunk_sum / chunk_count as f64;
            tempBin.push( Bin {mean: chunk_mean, sum: chunk_sum , min: chunk_min, max: chunk_max, count: chunk_count} );

            //println!("{} count: {} sum: {} min {} max {} SoS {} mean {}",cc, chunk_count, chunk_sum, chunk_min, chunk_max, chunk_sum_square, chunk_mean);
        });
        */
        tempBin 
    }


}