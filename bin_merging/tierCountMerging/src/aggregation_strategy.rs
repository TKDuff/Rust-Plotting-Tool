use crate::bin::Bin;
pub trait AggregationStrategy: Send + Sync {
    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (Bin, Bin);
    fn get_means(&self) -> Vec<[f64; 2]>;
}