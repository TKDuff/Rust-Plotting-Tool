use tokio::runtime::Runtime;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::time::{self, Duration, Interval};
use tokio::fs::File;
use tokio::sync::mpsc;

use std::thread;
use std::time::{ Instant};


fn main() {
    let rt = Runtime::new().unwrap();

    /*
    rt.block_on(async move {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            tokio::select! {
                line = lines.next_line() => {
                    if let Ok(Some(line)) = line {
                        println!("{}", line);
                        
                    } else {
                        // End of input or an error. You can break or handle it as needed.
                        break;
                    }
                }, 
            }
        }
    });*/

    rt.block_on(async move {
        let mut interval_count = 0;
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            tokio::select! { 
                _ = interval.tick() => {
                    priority_merge_dispatcher(interval_count)
                    interval_count += 1
                },
                _ = 

            }
            interval.tick().await;
            priority_merge_dispatcher(interval_count);
            
        }

    });

    pub fn priority_merge_dispatcher(elapsed: u64) {
        
        if elapsed % 1 == 0 {
            println!("Merging for Tier 1");
        }

        if elapsed % 1 == 0 {
            println!("Merging for Tier 2");
        }
    }
    

}