use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();          // Get the global stdin instance
    let locked_stdin = stdin.lock(); // Lock stdin for exclusive access


    /*
    Read lines from the locked stdin
    Uses iterator to read each line
    If line read succesful iterator return Ok(string) else Err(error)
    Each line in the loop iteration is of type "Result"
    Access string in Result via the unwrap() method
    */
    for line in locked_stdin.lines() {
        println!("{}", line.unwrap()); 
    }
}