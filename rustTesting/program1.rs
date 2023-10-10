use std::thread;
use std::time::Duration;

fn main() {
    /*
    let name: &str = "Alice";
    let age = 24;
    print_name(name, age)*/

    for i in 1..100 {
        println!("{} {}", i, i+1);
        thread::sleep(Duration::from_secs(1));
    }
}

/*
fn print_name(name: &str, age: i32){
    println!("My name is {} and I am {}", name, age);
}*/