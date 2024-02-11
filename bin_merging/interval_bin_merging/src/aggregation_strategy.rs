use crate::bin::Bin;  

pub trait AggregationStrategy: Send + Sync {
    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (f64, f64, usize);
    fn get_means(&self) -> Vec<[f64; 2]>;
    fn categorise_recent_bins(& mut self, seconds_interval: u128);
    fn merge_vector_bins(&self, bins: &[Bin], y: usize) -> Vec<Bin>;
}