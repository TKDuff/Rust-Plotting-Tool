use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};
use crate::aggregation_strategy::AggregationStrategy;
use std::time::{SystemTime, UNIX_EPOCH};
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
        println!("sum:{} length: {},mean_y: {}, mean_x:{} timestamp: {}", x_sum, x.len(), x_mean, x_mean, timestamp);

        (x_mean, y_mean, x.len())
    }

    fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }

    
    fn categorise_recent_bins(& mut self, seconds_interval: u128, seconds_length: usize) -> usize {
        /*You subtract 1 from the length of the bins vector to merge since you want to keep the final point in the plot consistent with the raw data*/
        if self.x_stats.is_empty() || (self.x_stats.len() == 1) {
            println!("x_stats is empty!");
            return 1;
        }
           
        let length: usize = self.x_stats.len();

        
        let merged_x_stats = self.merge_vector_bins(&self.x_stats[seconds_length..length - 1], 1);
        let merged_y_stats = self.merge_vector_bins(&self.y_stats[seconds_length..length - 1], 0);


        //self.x_stats.drain(self.seconds_length..length);
        print!("\nPre drain: ");
        for bin in &self.x_stats {
            print!("{}, ", bin.get_mean());
        }

        self.x_stats.drain(seconds_length..length - 1);
        self.x_stats.splice(seconds_length..seconds_length, merged_x_stats);

        self.y_stats.drain(seconds_length..length - 1);
        self.y_stats.splice(seconds_length..seconds_length, merged_y_stats);


        print!("\nPost drain: ");
        for bin in &self.x_stats {
            print!("{}, ", bin.get_mean());
        }
        println!("\nThe original length was {} the new lenght is {}", length, self.x_stats.len());
        println!("\n");

        //seconds_length = self.x_stats.len() - 1;
        self.x_stats.len() 
    }
    

    fn merge_vector_bins(&self, bins: &[Bin], c: i32) -> Vec<Bin> {

        if( c== 1){   
            print!("Pre merge:");
            for bin in bins {
                print!("{}, ", bin.get_mean());
            }
        }

        let mut tempBin: Vec<Bin> = Vec::new();

        
    
            // Calculate the sum and count for the current chunk
            let chunk_count: usize = bins.iter().map(|bin| bin.count).sum();
            let chunk_sum: f64 = bins.iter().map(|bin| bin.sum).sum();
            let chunk_min = bins.iter().map(|bin| bin.min).fold(f64::INFINITY, f64::min);
            let chunk_max = bins.iter().map(|bin| bin.max).fold(f64::NEG_INFINITY, f64::max);
            let chunk_mean = chunk_sum / chunk_count as f64;
            tempBin.push( Bin {mean: chunk_mean, sum: chunk_sum , min: chunk_min, max: chunk_max, count: chunk_count, timestamp: 0} );

            //println!("{} count: {} sum: {} min {} max {} SoS {} mean {}",cc, chunk_count, chunk_sum, chunk_min, chunk_max, chunk_sum_square, chunk_mean);

            if( c== 1){   
                print!("Post merge:");
                for bin in &tempBin {
                    print!("{}, ", bin.get_mean());
                }
            } 
        
        tempBin 
    }


}