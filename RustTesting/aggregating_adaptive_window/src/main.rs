mod adwin;

use std::io;
use csv::Reader;
use serde::Deserialize;
use std::io::{Read, Write};
use adwin::ADWIN;
use std::thread;
use std::time::Duration;


#[derive(Debug, Deserialize)]
struct Record {
    x_col: i32,
    y_col: f64,
}

fn main() -> io::Result<()> {
    
    //let file_path = "variance_dataset.csv";
    let file_path = "variance_dataset_low_100.csv";
    let mut rdr = Reader::from_path(file_path)?;

    let delta = 0.00000000000000000000000000000000000000002;
    let mut adaptive_window = ADWIN::new(delta);

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{}", adaptive_window.get_window().len());
        adaptive_window.add(record.y_col);
        thread::sleep(Duration::from_millis(100));
    }


    Ok(())
}