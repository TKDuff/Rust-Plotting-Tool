/*
use rand::{Rng};
use std::thread;
use std::time::Duration;
fn main() {
    let mut x = 0.0;
    loop {
        x += 5.0;
        //x += rand::thread_rng().gen_range(1.0..5.0);
        let y = rand::thread_rng().gen_range(1.0..10.0);
        println!("{} {}", x, y*y);
        //thread::sleep(Duration::from_millis(1));
    }  
}
*/



use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "x_col")]
    x_col: i32,
    #[serde(rename = "y_col")]
    y_col: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the CSV file
    let file_path: &str;
    //file_path = "dataset_500.csv";
    //file_path = "dataset_5000.csv";
    //file_path = "dataset_15000.csv";
    file_path = "dataset_100000.csv";

    // Create a new reader
    let mut rdr = Reader::from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{} {}", record.x_col, record.y_col);
        thread::sleep(Duration::from_millis(150));
    }

    Ok(())
}
//

//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/buffer_lttb/ && cargo run --bin buffer_lttb)
//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/buffer_test/ && cargo run --bin buffer_test)
//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/sliding_window_test/ && cargo run --bin sliding_window_test)
//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/combiningBins/ && cargo run --bin combiningBins)
//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/RustTesting/tokioTest/ && cargo run --bin tokioTest)
//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/RustTesting/aggregating_adaptive_window/ && cargo run --bin aggregating_adaptive_window)