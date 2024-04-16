use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "y_col")]
    y_col: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the CSV file
    let file_path: &str;
    let mut count: i32 = 0;

    //file_path = "powerconsumption_zone1.csv";
    //file_path = "powerconsumption_zone2.csv";
    //file_path = "/home/FinalYearProject/online-graph/pow_cons/powerconsumption_zone3.csv";
    file_path = "/home/thomas/FinalYearProject/online-graph/pow_cons/target/release/powerconsumption_zone3.csv";



    //file_path = "MetroPT3(AirCompressor).csv";

    // Create a new reader
    let mut rdr = Reader::from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{} {}", count, record.y_col);
        count += 1;
        thread::sleep(Duration::from_millis(2));
    }

    Ok(())
}