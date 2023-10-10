use std::io::{self, BufRead};
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    //using 'move' to move tx into the closure so the spawned thread owns tx
    let reader = thread::spawn(move || {
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
            let line_string = line.unwrap();
            tx.send(line_string).unwrap();
            /*send() method returns Result, this result used as error handling as if receiving thread not exist will print error, hence why we unwrap() the send
            If send succesful, the Result return Ok(()), no value to extract
            */
        }
    });

    let processer = thread::spawn(move || {
        for line in rx {
            println!("Test {}", line);
        }
    });

    reader.join().unwrap();
    processer.join().unwrap();
}