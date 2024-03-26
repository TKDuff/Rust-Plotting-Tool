use crate::data_strategy::DataStrategy;
use statrs::statistics::{Data, Min, Max,Distribution};
use crate::bin::Bin;

pub struct CountRawData {
    pub condition: usize,
    pub points: Vec<[f64;2]>,
}

impl CountRawData {
    pub fn new(condition: usize) -> Self {
        Self {

            condition: condition,
            points: Vec::new(),
        }
    }

}

impl DataStrategy for CountRawData {

    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (Bin, Bin, Bin, Bin) {
        /*Aggregates a chunk of x,y values into two bins, x bin and y bin respectively */

        let chunk_len = chunk.len();

        /*Splits the 2D vector into two seperate vectors, for the x values and y values
        It was writtein with the aid of chatGPT[1] (see reference at bottom), I changed it to work with my vector
         */
        let (x_vec, y_vec): (Vec<f64>, Vec<f64>) = chunk.iter() //iterate over the chunk of points
                                                .take(chunk_len.saturating_sub(1)) //subtract sub excludes the final point, need to return final point as bin
                                                .map(|&[x, y]| (x, y))  //map function called 2D vector, maps each x,y array element to tuple
                                                .unzip(); //unzip seperates tuples into two seperate vectors, x_vec and y_vec

        //create Data objects for statrs library
        let x = Data::new(x_vec.clone());   
        let y = Data::new(y_vec.clone());

        //calculate the means
        let x_mean =  x.mean().unwrap();
        let y_mean = y.mean().unwrap();

        //let agg_x_bin = Bin {mean: x_mean, sum: x.iter().sum() , min: x.min(), max: x.max(), count: x.len(), sum_of_squares: x_sum_of_squares, variance: x_variance, standard_deviation: x_variance.sqrt() };
        let agg_x_bin = Bin::new(x_mean, x.iter().sum() , x.min(), x.max(), x.len() );
        let agg_y_bin = Bin::new(y_mean, y.iter().sum() , y.min(), y.max(), y.len() ); 

        //println!("{} {} {} {} {} {} {} {} {} {} {}", agg_x_bin.mean, agg_x_bin.sum, agg_x_bin.min, agg_x_bin.max, agg_x_bin.count, agg_x_bin.sum_of_squares, agg_x_bin.variance, agg_x_bin.standard_deviation, agg_x_bin.range, agg_x_bin.estimated_q1, agg_x_bin.estimated_q3);

        //since initial tiers last element the un-aggregated x,y value, they are returned in the form of a bin to be placed in the last element of the initial tier
        let last_elem_x_bin = Bin::new(chunk[chunk_len-1][0], 0.0, 0.0, 0.0, 0);
        let last_elem_y_bin = Bin::new(chunk[chunk_len-1][1], 0.0, 0.0, 0.0, 0);


        (last_elem_x_bin, last_elem_y_bin, agg_x_bin, agg_y_bin)
    }

    fn append_str(&mut self, line:String) {
        println!("{}", line);


        let values_result: Result<Vec<f64>, _> = line.split(' ')
        .map(|s| s.trim().parse::<f64>())
        .collect();

        match values_result {
            Ok(values) => {
                self.append_point(values[0], values[1]);
            }
            Err(err) => {
                println!("Error parsing x y values: {:?}", err);
            }
        }
    }

    fn get_raw_data(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }

    fn append_point(&mut self, x_value: f64, y_value: f64) {
        self.points.push([x_value, y_value]);
    }

    fn get_length(&self) -> usize {
        self.points.len()
    }

    fn remove_chunk(&mut self, count:usize) {
        self.points.drain(..count);
    }

    fn check_cut(&self) -> Option<usize> {
        if (self.points.len() > 1) && (self.points.len() - 1) % self.condition == 0 {
            return Some(self.points.len() -1 )
        } else {
            None
        }
    }

    fn get_chunk(&self, count:usize) -> Vec<[f64;2]> {
        self.points[0..count + 1].to_vec()
    }

    fn get_condition(&self) -> usize {
        self.condition
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_chunk_aggregate_statistics() {
        let test_data = vec![
            [-3.040, -2.727],
            [-1.292, -1.127],
            [0.063, 1.512],
            [3.596, -2.051],
            [-4.049, 0.037],
            [2.733, -3.023],
            [-1.836, -3.216],
            [-0.795, -2.082],
            [1.842, 2.255],
            [-0.908, 4.849],
            [0.0, 0.0],
            [3.333, 3.333],
            [-2.222, -2.222],
            [1000.123, 2000.321],
            [1.111, 0.0],  //this element is excluded from the calculations, but still passed to the metho
        ];

        let mut stdin_tier_instance = CountRawData::new(0);

        // Act
        let (last_elem_x_bin, last_elem_y_bin, agg_x_bin, agg_y_bin) = stdin_tier_instance.append_chunk_aggregate_statistics(test_data);

        // Assert
        assert!((agg_x_bin.mean -  71.253).abs() < 1e-3); //check for assert with condition that floats are within tolerance of 0.001
        assert_eq!(agg_x_bin.sum, 997.548);
        assert_eq!(agg_x_bin.min, -4.049);
        assert_eq!(agg_x_bin.max, 1000.123);
        assert_eq!(agg_x_bin.count, 14);
        assert!((agg_x_bin.range - 1004.172).abs() < 1e-3);
        assert!((agg_x_bin.estimated_q1 - -179.78957).abs() < 1e-3);
        assert!((agg_x_bin.estimated_q3 - 322.2964).abs() < 1e-3);
        assert!((agg_y_bin.mean -  142.561).abs() < 1e-3);
        assert_eq!(agg_y_bin.sum, 1995.859);
        assert_eq!(agg_y_bin.min, -3.216);
        assert_eq!(agg_y_bin.max, 2000.321);
        assert_eq!(agg_y_bin.count, 14);
        assert!((agg_y_bin.range - 2003.537).abs() < 1e-3);
        assert!((agg_y_bin.estimated_q1 - -358.32289).abs() < 1e-3);
        assert!((agg_y_bin.estimated_q3 - 643.44560).abs() < 1e-3);
    }
}


/*
[1] - ChatGPT version 4, default setting, Prompt "In Rust, how do you split a 2 dimensional vector into two separate vectors? The vector is made up of 2D arrays, like Vec<[f64:2]>"

*/