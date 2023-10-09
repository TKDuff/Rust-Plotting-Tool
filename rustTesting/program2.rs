use std::io::{Read}; //import standard input/output, need 'Read' trait to read from byte stream

fn main(){
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
    /*
    std::io::stdin() - the stanard input stream
    .read_to_string(&mut buffer) - reads to end of standard input, appends to buffer
    .expect("Failed to read from stdin") - read_to_string() returns 'Result' which is either Ok or Err, if Err will prints 

    read_to_string() return type is Result
    Result: represent outcome of operation that can pass or fail
    Results has two enum variables, Ok and Err

    expect() is method ran on the Result enum variable Ok or Err, so calling Result.except() is calling either Ok.except() or Err.except()

    */

    println!("{}", buffer) //print buffer as string literal

}