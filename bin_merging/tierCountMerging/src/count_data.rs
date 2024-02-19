use crate::data_strategy::DataStrategy;

pub struct CountRawData {
    pub points_count: usize,
    pub points: Vec<[f64;2]>,
}

impl CountRawData {
    pub fn new() -> Self {
        Self {

            //When set to 10 ecluding the last point, as that is kept for plot consistency
            points_count: 5,
            points: Vec::new(),//vec![[0.0, 0.0]]
        }
    }

}

impl DataStrategy for CountRawData {


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
        //self.points[0] = [point_means.0, point_means.1];
        self.points.drain(0..count-1);
    }

    fn check_cut(&self) -> Option<Vec<[f64;2]>> {
        if self.points.len() > 1 && (self.points.len()) % self.points_count == 0 {
            //return Some(self.points.len() -1 )
            return Some(self.points[0..self.points.len()].to_vec())
        } else {
            None
        }
    }

    
    // fn get_chunk(&self, count:usize) -> Vec<[f64;2]> {
    //     self.points[1..count+1].to_vec()
    // }
}