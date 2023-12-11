use rand::{Rng};
use std::thread;
use std::time::Duration;

fn main() {
    let mut x = 0.0;
    loop {
        //x += 0.7;
        x += rand::thread_rng().gen_range(1.0..5.0);
        let y = rand::thread_rng().gen_range(1.0..50.0);
        println!("{} {}", x, y);
        thread::sleep(Duration::from_millis(250));
    }  
}


//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/buffer_test/ && cargo run --bin buffer_test)
//cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/sliding_window_test/ && cargo run --bin sliding_window_test)
