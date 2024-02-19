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

    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (Bin, Bin) {
        //let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().map(|&[x, y]| (x, y)).unzip();
        //println!("Aggregating this chunk {:?}\n", chunk); 

        let chunk_len = chunk.len();
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
        

        self.x_stats.push(Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len() });
        self.y_stats.push(Bin {mean: y_mean, sum: y.iter().sum() , min: y.min(), max: y.max(), count: y.len() });

        println!("The sum is: {} The length is: {}, The y mean is {}, The x mean is {}", y_sum, y.len(), y_mean, x_mean);


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

        println!("X: {:?}, Y: {:?}", x_bin, y_bin);

        (x_bin, y_bin)
    }

    fn get_means(&self) -> Vec<[f64; 2]> {
        self.x_stats.iter().zip(self.y_stats.iter())
            .map(|(x, y)| [x.mean, y.mean]) // Assuming index 5 is the mean
            .collect()
    }
}