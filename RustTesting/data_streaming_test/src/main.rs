use incr_stats::incr::Stats;
use std::io::{self, BufRead};

fn main() {
    let mut stats = Stats::new();
    let stdin = io::stdin();

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
    }
}

//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/RustTesting/data_streaming_test/ && cargo run --bin data_streaming_test)