pub trait DataStrategy: Send + Sync {
    fn append_str(&mut self, line:String);
    fn get_raw_data(&self) -> Vec<[f64; 2]>;
    fn append_point(&mut self, x_value: f64, y_value: f64);
    fn requires_external_trigger(&self) -> bool;
    fn get_values(&self) -> Vec<[f64; 2]>;
    fn remove_chunk(&mut self, count:usize, point_means: (f64, f64));
    fn check_cut(&self) -> Option<usize>;
    fn get_chunk(&self, count:usize) -> Vec<[f64;2]>;
}