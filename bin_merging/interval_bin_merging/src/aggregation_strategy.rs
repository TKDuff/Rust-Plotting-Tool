use crate::bin::Bin;  

pub trait AggregationStrategy: Send + Sync {
    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (f64, f64, usize);
    fn get_means(&self) -> Vec<[f64; 2]>;
    fn merge_x(&self) -> Vec<Bin>;
    fn merge_y(&self) -> Vec<Bin>;
    fn drain_x(&mut self);
    //fn categorise_recent_bins(& mut self, seconds_interval: u128, seconds_length: usize) -> usize;
    //fn merge_vector_bins(&self, bins: &[Bin], c:i32) -> Vec<Bin>;
}