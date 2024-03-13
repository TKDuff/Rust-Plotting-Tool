use crate::bin::Bin;
use std::time::Duration;


pub trait DataStrategy: Send + Sync {
    fn append_str(&mut self, line:String, start: std::time::Instant, total_duration: &mut std::time::Duration);
    fn get_raw_data(&self) -> Vec<[f64; 2]>;
    fn append_point(&mut self, x_value: f64, y_value: f64);
    fn requires_external_trigger(&self) -> bool;
    fn get_values(&self) -> Vec<[f64; 2]>;
    fn remove_chunk(&mut self, count:usize);
    fn check_cut(&self) -> Option<usize>;
    fn get_length(&self) -> usize;
    fn get_chunk(&self, count:usize) -> Vec<[f64;2]>;
    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (Bin, Bin, Bin, Bin);
    fn get_condition(&self) -> usize;
}