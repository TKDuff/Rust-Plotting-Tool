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
        thread::sleep(Duration::from_millis(500));
    }  
}*/




use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Episode No.")]
    episode_no: i32,
    #[serde(rename = "Reward")]
    reward: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the CSV file
    let file_path = "Generated_Reward_Data_10000.csv";

    // Create a new reader
    let mut rdr = Reader::from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{} {}", record.episode_no, record.reward);
        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}



//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/buffer_test/ && cargo run --bin buffer_test)
//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/sliding_window_test/ && cargo run --bin sliding_window_test)
