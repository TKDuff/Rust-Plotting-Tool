use std::io::{self, BufRead};
use std::thread;
use std::sync::mpsc;
use std::io::Write;

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
            If send succesful,returned Result value Ok(()), no value to extract
            */
        }
    });

    let processer = thread::spawn(move || {
        println!("\n\n\n");
        let mut total_x_value = 0.0;
        let mut total_y_value = 0.0;
        let mut count = 0.0;

        
        for line in rx {
            let parts: Vec<&str> = line.split_whitespace().collect();
            total_x_value += parts[0].parse::<f32>().unwrap();
            total_y_value += parts[1].parse::<f32>().unwrap();
            count += 1.0;
            print!("\x1B[4A\rx value: {}\n\rx average: {}\n\n\ry value: {}\n\ry average: {}", parts[0], (total_x_value/count), parts[1], (total_y_value/count));
            std::io::stdout().flush().unwrap();

        }
    });

    reader.join().unwrap();
    processer.join().unwrap();
}