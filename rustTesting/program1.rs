use std::thread;
use std::time::Duration;

fn main() {
    for i in 1..100 {
        println!("{} {}", i, i+1);
        thread::sleep(Duration::from_secs(1));
    }
}
