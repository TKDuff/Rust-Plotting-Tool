pub trait AggregationStrategy: Send + Sync {
    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>) -> (f64, f64, usize);
    fn get_means(&self) -> Vec<[f64; 2]>;
}