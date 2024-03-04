use crate::data_strategy::DataStrategy;
use statrs::statistics::{Data, OrderStatistics, Min, Max,Distribution};
use crate::bin::Bin;

pub struct CountRawData {
    pub condition: usize,
    pub points: Vec<[f64;2]>,
}

impl CountRawData {
    pub fn new(condition: usize) -> Self {
        Self {

            //When set to 10 ecluding the last point, as that is kept for plot consistency
            condition: condition,
            points: Vec::new(),//vec![[0.0, 0.0]]
        }
    }

}

impl DataStrategy for CountRawData {

    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (Bin, Bin, Bin, Bin) {
        //println!("Aggregating this chunk {:?}\n", chunk); 

        let chunk_len = chunk.len();
        //let stats_len = self.x_stats.len();
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter()
                                                .take(chunk_len.saturating_sub(1)) //subtract sub excludes the final point, need to return final point as bin
                                                .map(|&[x, y]| (x, y))
                                                .unzip();

        let x = Data::new(x_vec.clone());   
        let y = Data::new(y_vec.clone());

        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        let x_sum_of_squares: f64 = x_vec.iter().map(|&x| (x - x_mean).powi(2)).sum();
        let y_sum_of_squares: f64 = y_vec.iter().map(|&y| (y - y_mean).powi(2)).sum();

        let x_variance: f64 = x_sum_of_squares / (x.len() as f64 - 1.0);
        let y_variance: f64 = y_sum_of_squares / (x.len() as f64 - 1.0);

        let y_sum: f64 = y_vec.iter().sum();

        let agg_x_bin = Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len(), sum_of_squares: x_sum_of_squares, variance: x_variance, standard_deviation: x_variance.sqrt() };
        let agg_y_bin = Bin {mean: y_mean, sum: y.iter().sum() , min: y.min(), max: y.max(), count: y.len(), sum_of_squares: y_sum_of_squares, variance: y_variance, standard_deviation: y_variance.sqrt() };

        //println!("The sum is: {} The length is: {}, The y mean is {}, The x mean is {}", y_sum, y.len(), y_mean, x_mean);
        let last_elem_x_bin = Bin {
            mean: chunk[chunk_len-1][0],
            sum: 0.0,
            min: 0.0,
            max: 0.0,
            count: 0,
            sum_of_squares: 0.0,
            variance: 0.0,
            standard_deviation: 0.0,
        };
    
        let last_elem_y_bin = Bin {
            mean: chunk[chunk_len-1][1],
            sum: 0.0,
            min: 0.0,
            max: 0.0,
            count: 0,
            sum_of_squares: 0.0,
            variance: 0.0,
            standard_deviation: 0.0,
        };
        (last_elem_x_bin, last_elem_y_bin, agg_x_bin, agg_y_bin)
    }

    fn append_str(&mut self, line:String) {
        let values_result: Result<Vec<f64>, _> = line.split(' ')
        .map(|s| s.trim().parse::<f64>())
        .collect();

        match values_result {
            Ok(values) => {
                //println!("{} {}", values[0], values[1]);
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
        //println!("Before removing {:?}", self.points);
        self.points.drain(..count);
        //println!("After removing {:?}", self.points);
    }

    fn check_cut(&self) -> Option<usize> {
        if (self.points.len() > 1) && (self.points.len() - 1) % self.condition == 0 {
            return Some(self.points.len() -1 )
        } else {
            None
        }
    }

    fn get_chunk(&self, count:usize) -> Vec<[f64;2]> {

        self.points[0..count+1].to_vec()
    }

    fn get_condition(&self) -> usize {
        self.condition
    }


}