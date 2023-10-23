use rand::{Rng};
use std::thread;
use std::time::Duration;

fn main() {
    let mut x = 0;
    loop {
        x += 5;
        let y = rand::thread_rng().gen_range(50..60);
        println!("{} {}", x, y);
        thread::sleep(Duration::from_millis(100));
    }  
}
