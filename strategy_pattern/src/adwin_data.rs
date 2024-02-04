use crate::data_strategy::DataStrategy;

pub struct AdwinRawData {
    pub points: Vec<[f64;2]>,
}

impl AdwinRawData {
    pub fn new() -> Self {
        Self { points: vec![[0.0, 0.0]] }
    }
}

impl DataStrategy for AdwinRawData {
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

    fn remove_chunk(&mut self, count:usize, point_means: (f64, f64)) {
        self.points[0] = [point_means.0, point_means.1];
        self.points.drain(1..count+1);
    }
}