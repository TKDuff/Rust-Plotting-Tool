use std::io::{self, BufRead};
use std::thread;

fn main() {


    let consumer = thread::spawn(|| {
        //Expensive initialisation inside thread here
        let stdin = io::stdin();          //global stdin instance
        let locked_stdin = stdin.lock();  //lock stdin for exclusive access

        /*
        Read lines from the locked stdin
        Uses iterator to read each line
        If line read succesful iterator return Ok(string) else Err(error)
        Each line in the loop iteration is of type "Result"
        Access string in Result via the unwrap() method*/
        for line in locked_stdin.lines() {
            println!("Thread print {}", line.unwrap()); 
        }
    });

    consumer.join().unwrap();   //main thread will wait for the consumer thread to finish, so program won't close until consumer stop (program 1 prints all numbers)
}