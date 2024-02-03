
pub struct adwin_aggregate {
    aggregate_points: Vec<[f64;2]>,
}

pub fn get_aggregate_points(&self) -> Vec<[f64; 2]> {
    self.aggregate_points.clone().into_iter().collect()
}

fn compute_vector_means(&self, split_index: usize, window_y_values: &[f64]) -> (f64, f64) {
    let sum1: f64 = window_y_values.iter().take(split_index).sum();
    let mean1 = sum1 / split_index as f64;

    let sum2: f64 = window_y_values.iter().skip(split_index).sum();
    let mean2 = sum2 / (window_y_values.len() - split_index) as f64;
    (mean1, mean2)
}