use incr_stats::incr::Stats;
use std::io::{self, BufRead};
use average::Variance;


fn main() {
    let mut stats = Stats::new();
    let stdin = io::stdin();

    let data1 = vec![500.0,600.0,550.0];
    let data2 = vec![500.0];

    let n1 = data1.len() as f64;
    let n2 = data2.len() as f64;

    // Convert the data vector into a Variance object
    let v1 = data1.into_iter().collect::<Variance>();
    let v2 = data2.into_iter().collect::<Variance>();


    // Calculate the sample variance
    let SV1 = v1.sample_variance();
    let SV2 = v1.sample_variance();

    let w_v1 = n1 * SV1;
    let w_v2 = n2 * SV2;

    println!("Sample Variance: {} {}", w_v1, w_v2);

    /*
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();


        let x: i32 = parts[0].parse().expect("Expected x to be an integer");
        let y: f64 = parts[1].parse().expect("Expected y to be a floating point number");

        stats.update(y).expect("Failed to update stats");

        if stats.count() >= 2 {
            println!("For x = {}, current y variance: {}", x, stats.sample_variance().unwrap());
        } else {
            println!("For x = {}, need more data for variance calculation", x);
        }
    }*/
}

//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/RustTesting/data_streaming_test/ && cargo run --bin data_streaming_test)