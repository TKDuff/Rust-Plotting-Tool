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
            x_stats: vec![Bin::default()],
            y_stats: vec![Bin::default()],
        }
    }

    pub fn get_x_means(&self) -> Vec<f64> {
        self.x_stats.iter().map(|bin| bin.get_mean()).collect()
    }  

}

impl AggregationStrategy for CountAggregateData {

    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) {
        //let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().map(|&[x, y]| (x, y)).unzip();
        //println!("Aggregating this chunk {:?}\n", chunk); 

        let chunk_len = chunk.len();
        let stats_len = self.x_stats.len();
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter()
                                                .take(chunk_len.saturating_sub(1)) //subtract sub excludes the final point, need to return final point as bin
                                                .map(|&[x, y]| (x, y))
                                                .unzip();

        //println!("\nAggregating this chunk {:?}", x_vec);                            


        let x = Data::new(x_vec.clone());   
        let y = Data::new(y_vec.clone());

        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        let y_sum: f64 = y_vec.iter().sum();
        

        //self.x_stats.push(Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len() });
        //self.y_stats.push(Bin {mean: y_mean, sum: y.iter().sum() , min: y.min(), max: y.max(), count: y.len() });

        self.x_stats[stats_len-1] = Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len() };
        self.y_stats[stats_len-1] = Bin {mean: y_mean, sum: y.iter().sum() , min: y.min(), max: y.max(), count: y.len() };

        //println!("The sum is: {} The length is: {}, The y mean is {}, The x mean is {}", y_sum, y.len(), y_mean, x_mean);
        println!("The x mean is {}",x_mean);
        let x_bin = Bin {
            mean: chunk[chunk_len-1][0],
            sum: 0.0,
            min: 0.0,
            max: 0.0,
            count: 0,
        };
    
        let y_bin = Bin {
            mean: chunk[chunk_len-1][1],
            sum: 0.0,
            min: 0.0,
            max: 0.0,
            count: 0,
        };

        println!("The last r.d element is {}", x_bin.mean);
        self.x_stats.push(x_bin);
        self.y_stats.push(y_bin);

        println!("X means {:?}", self.get_x_means());

    }

    fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }

    fn get_length(&self) -> usize {
        self.x_stats.len()
    }


    fn merge_vector_bins(&self, bins: &[Bin]) -> Vec<Bin> {
        if self.x_stats.is_empty() || (self.x_stats.len() == 1) {
            println!("x_stats is empty!");
        }
        let mut temp_bin: Vec<Bin> = Vec::new();
        
        // Calculate the sum and count for the current chunk
        let chunk_count: usize = bins.iter().map(|bin| bin.count).sum();
        let chunk_sum: f64 = bins.iter().map(|bin| bin.sum).sum();
        let chunk_min = bins.iter().map(|bin| bin.min).fold(f64::INFINITY, f64::min);
        let chunk_max = bins.iter().map(|bin| bin.max).fold(f64::NEG_INFINITY, f64::max);
        let chunk_mean = chunk_sum / chunk_count as f64;
        temp_bin.push( Bin {mean: chunk_mean, sum: chunk_sum , min: chunk_min, max: chunk_max, count: chunk_count} );
        
        temp_bin 
    }

    fn get_slices(&self, length: usize) -> (&[Bin], &[Bin])  {
        let x_slice = &self.x_stats[0..std::cmp::min(length, self.x_stats.len())];
        let y_slice = &self.y_stats[0..std::cmp::min(length, self.y_stats.len())];

        (x_slice, y_slice)
    }

    // fn misc_x(&self, average: Bin, length: usize) {
    //     self.x_stats[0] = average;
    //     self.x_stats.drain(1..length-1);

    // }

    // fn misc_y(&self, average: Bin, length: usize) {
    //     self.y_stats[0] = average;
    //     self.y_stats.drain(1..length-1);
    // }
}

