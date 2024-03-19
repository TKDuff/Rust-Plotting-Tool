use std::time::Duration;

use crate::data_strategy::DataStrategy;
use statrs::statistics::{Data, Min, Max,Distribution};
use crate::bin::Bin;

pub struct IntervalRawData {
    pub condition: usize,
    pub points: Vec<[f64;2]>,
    pub time: usize
}

impl IntervalRawData {
    pub fn new(condition: usize) -> Self {
        Self { 
            condition: condition,
            points: Vec::new(),
            time: 0,

        }
    }


    pub fn get_x(&self) -> Vec<f64> {
        self.points.iter().map(|&arr| arr[0]).collect()
    }
}

impl DataStrategy for IntervalRawData {

    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (Bin, Bin, Bin, Bin) {
        //let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter().map(|&[x, y]| (x, y)).unzip();
        //println!("Aggregating this chunk {:?}\n", chunk); 

        let chunk_len = chunk.len();
        //let stats_len = self.x_stats.len();
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter()
                                                .take(chunk_len.saturating_sub(1)) //subtract sub excludes the final point, need to return final point as bin
                                                .map(|&[x, y]| (x, y))
                                                .unzip();

        //println!("Aggregating this chunk non last {:?}\n", x_vec);                            


        let x = Data::new(x_vec.clone());   
        let y = Data::new(y_vec.clone());

        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        //let agg_x_bin = Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len(), sum_of_squares: x_sum_of_squares, variance: x_variance, standard_deviation: x_variance.sqrt() };
        let agg_x_bin = Bin::new(x_mean, x.iter().sum() , x.min(), x.max(), x.len() );
        let agg_y_bin = Bin::new(y_mean, y.iter().sum() , y.min(), y.max(), y.len() ); 

        
        let last_elem_x_bin = Bin::new(chunk[chunk_len-1][0], 0.0, 0.0, 0.0, 0);
        let last_elem_y_bin = Bin::new(chunk[chunk_len-1][1], 0.0, 0.0, 0.0, 0);

        //println!("last X {:?}\nlast Y {:?}\nMerged X {:?}\nMerged Y {:?}", last_elem_x_bin, last_elem_y_bin, agg_x_bin, agg_y_bin);
        (last_elem_x_bin, last_elem_y_bin, agg_x_bin, agg_y_bin)

        //println!("X means {:?}", self.get_x_means());

    }



    fn append_str(&mut self, line:String) {
        let values_result: Result<Vec<f64>, _> = line.split(' ')
        .map(|s| s.trim().parse::<f64>())
        .collect();

        match values_result {
            Ok(values) => {
                self.append_point(values[0], values[1]);
            }
            Err(err) => {
                println!("Error parsing values: {:?}", err);
            }
        }
    }

    fn append_point(&mut self, x_value: f64, y_value: f64) {
        self.points.push([x_value, y_value]);
    }

    fn get_raw_data(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }

    fn get_length(&self) -> usize {
        self.points.len()
    }
    
    /*
    Interval remove chunk decrement length by 1 since 'count' determined at tick is all the elements appedned to the raw data vector so far, including the final point
    In count the remove_chunk does not decrement by 1 since the check_cut method checks the length of the rae data vector minus 1 
     */
    fn remove_chunk(&mut self, count:usize) {
        //println!("Before removing {:?}", self.points);
        self.points.drain(..count-1);
        //println!("After removing {:?}", self.points);
    }

    fn check_cut(&self) -> Option<usize> {
        unreachable!();
    }

    fn get_chunk(&self, count:usize) -> Vec<[f64;2]> {
        self.points.clone().into_iter().collect()
    }

    fn get_condition(&self) -> usize {
        self.condition
    }

    fn increment_time(&mut self) {
        self.time += 1;
    }

    fn get_time(&self) -> Option<usize> {
        Some(self.time)
    }
}