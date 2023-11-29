use rand::{Rng};
use std::thread;
use std::time::Duration;

fn main() {
    let mut x = 0.0;
    loop {
        x += 0.1;
        //x += rand::thread_rng().gen_range(0.1..0.5);
        let y = rand::thread_rng().gen_range(0.50..0.60);
        println!("{} {}", x, y);
        thread::sleep(Duration::from_millis(100));
    }  
}
