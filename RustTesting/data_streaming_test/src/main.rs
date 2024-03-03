use incr_stats::incr::Stats;
use std::io::{self, BufRead};
use average::Variance;


fn main() {
    let mut full_data: Vec<f64> = vec![3.0, 4.0, 5.0, 3.0, 2.0, 1.0, 2.0, 4.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 4.0, 3.0];

    let mut bin1 = &full_data[..8];
    let mut bin2 = &full_data[8..];

    println!("1 {:?}", bin1);
    println!("2 {:?}", bin2);

    let variance_full = calculate_variance(&full_data);
    let variance1 = calculate_variance(bin1);
    let variance2 = calculate_variance(bin2);

    let mean1: f64 = bin1.iter().sum::<f64>() / bin1.len() as f64;
    let mean2: f64 = bin2.iter().sum::<f64>() / bin2.len() as f64;

    let combined_variance = ((bin1.len() as f64 - 1.0) * variance1 + (bin2.len() as f64 - 1.0) * variance2 
                            + (bin1.len() as f64 * bin2.len() as f64) / (bin1.len() as f64 + bin2.len() as f64) * (mean1 - mean2).powi(2))
                            / (bin1.len() as f64 + bin2.len() as f64 - 1.0);


    println!("Variance of Full Dataset: {}", variance_full);
    println!("Variance of Bin 1: {}", variance1);
    println!("Variance of Bin 2: {}", variance2);
    println!("Combined Variance: {}", combined_variance);

}

fn calculate_variance(data: &[f64]) -> f64 {
    let mean: f64 = data.iter().sum::<f64>() / data.len() as f64;
    let sum_of_squares: f64 = data.iter().map(|&value| (value - mean).powi(2)).sum();
    sum_of_squares / (data.len() as f64 - 1.0)
}
