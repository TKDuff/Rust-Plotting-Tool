use crate::data_strategy::DataStrategy;

pub struct StdinData {
    pub points: Vec<[f64;2]>,
}

impl StdinData {
    pub fn new() -> Self {
        Self { points: vec![[0.0, 0.0]] }
    }
}

impl DataStrategy for StdinData {
    fn append_str(&mut self, line:String) {
        let values_result: Result<Vec<f64>, _> = line.split(' ')
        .map(|s| s.trim().parse::<f64>())
        .collect();

        match values_result {
            Ok(values) => {
                self.append_points(values[0], values[1]);
            }
            Err(err) => {
                println!("Error parsing values: {:?}", err);
            }
        }
    }

    fn get_raw_data(&self) -> Vec<[f64; 2]> {
        self.points.clone().into_iter().collect()
    }

    fn append_points(&mut self, x_value: f64, y_value: f64) {
        self.points.push([x_value, y_value]);
    }
}