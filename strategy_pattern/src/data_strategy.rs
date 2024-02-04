pub trait DataStrategy {
    fn append_str(&mut self, line:String);
    fn get_raw_data(&self) -> Vec<[f64; 2]>;
    fn append_points(&mut self, x_value: f64, y_value: f64);
}