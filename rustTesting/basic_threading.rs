/*import standard input/output, self saves typing "std::io::stdin()" can just type io::stdin(), 'Read' trait to read from byte stream*/
use std::io::{self, Read};
use std::thread;


fn main(){
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
    /*
    std::io::stdin() - the stanard input stream
    .read_to_string(&mut buffer) - reads to end of standard input, appends to buffer
    .expect("Failed to read from stdin") - read_to_string() returns 'Result' which is either Ok or Err, if Err will prints 

    read_to_string() return type is Result
    Result: represent outcome of operation that can pass or fail
    Results has two enum variables, Ok and Err

    expect() is method ran on the Result enum variable Ok or Err, so calling Result.except() is calling either Ok.except() or Err.except()

    */
    println!("{}", buffer); //print buffer as string literal

    
    let handle1 = thread::spawn(|| {
        //println!("New Thread");
        for i in 1..10{
            println!("{}", i);
        }
    });

    let handle2 = thread::spawn(|| {
        //println!("New Thread");
        for i in 10..20{
            println!("{}", i);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();


}