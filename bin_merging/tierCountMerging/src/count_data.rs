use crate::data_strategy::DataStrategy;
use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};
use crate::bin::Bin;

pub struct CountRawData {
    pub points_count: usize,
    pub points: Vec<[f64;2]>,
}

impl CountRawData {
    pub fn new() -> Self {
        Self {

            //When set to 10 ecluding the last point, as that is kept for plot consistency
            points_count: 7,
            points: Vec::new(),//vec![[0.0, 0.0]]
        }
    }

}

impl DataStrategy for CountRawData {

    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (Bin, Bin, Bin, Bin) {
        //let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().map(|&[x, y]| (x, y)).unzip();
        //println!("Aggregating this chunk {:?}\n", chunk); 

        let chunk_len = chunk.len();
        //let stats_len = self.x_stats.len();
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

        let agg_x_bin = Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len() };
        let agg_y_bin = Bin {mean: y_mean, sum: y.iter().sum() , min: y.min(), max: y.max(), count: y.len() };

        //println!("The sum is: {} The length is: {}, The y mean is {}, The x mean is {}", y_sum, y.len(), y_mean, x_mean);
        println!("The x mean is {}",x_mean);
        let last_elem_x_bin = Bin {
            mean: chunk[chunk_len-1][0],
            sum: 0.0,
            min: 0.0,
            max: 0.0,
            count: 0,
        };
    
        let last_elem_y_bin = Bin {
            mean: chunk[chunk_len-1][1],
            sum: 0.0,
            min: 0.0,
            max: 0.0,
            count: 0,
        };

        println!("The last r.d element is {}", last_elem_x_bin.mean);
        
        (last_elem_x_bin, last_elem_y_bin, agg_x_bin, agg_y_bin)

        //println!("X means {:?}", self.get_x_means());

    }

    fn append_str(&mut self, line:String) {
        let values_result: Result<Vec<f64>, _> = line.split(' ')
        .map(|s| s.trim().parse::<f64>())
        .collect();

        match values_result {
            Ok(values) => {
                println!("{}", values[0]);
                self.append_point(values[0], values[1]);
            }
            Err(err) => {
                println!("Error parsing values: {:?}", err);
            }
        }
    }

    fn get_raw_data(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }

    fn append_point(&mut self, x_value: f64, y_value: f64) {
        self.points.push([x_value, y_value]);
    }

    fn requires_external_trigger(&self) -> bool {
        false
    }

    fn get_values(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }

    fn get_length(&self) -> usize {
        self.points.len()
    }

    fn remove_chunk(&mut self, count:usize) {
        self.points.drain(..count);
        //println!("After removing {:?}", self.points);
    }

    fn check_cut(&self) -> Option<usize> {
        if (self.points.len() > 1) && (self.points.len() - 1) % self.points_count == 0 {
            return Some(self.points.len() -1 )
        } else {
            None
        }
    }

    fn get_chunk(&self, count:usize) -> Vec<[f64;2]> {

        self.points[0..count+1].to_vec()
    }

}