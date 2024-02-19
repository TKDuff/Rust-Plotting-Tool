use crate::bin::Bin;
pub trait AggregationStrategy: Send + Sync {
    fn append_chunk_aggregate_statistics(&mut self, chunk: Vec<[f64;2]>);
    fn get_means(&self) -> Vec<[f64; 2]>;
    fn get_length(&self) -> usize;
    fn merge_vector_bins(&self, bins: &[Bin]) -> Vec<Bin>;
    fn get_slices(&self, length: usize) -> (&[Bin], &[Bin]) ;
    // fn misc_x(&self, average: Vec<Bin>, length: usize);
    // fn misc_y(&self, average: Vec<Bin>, length: usize);
}