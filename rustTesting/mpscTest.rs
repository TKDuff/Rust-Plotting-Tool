use std::thread;
use std::sync::mpsc;

fn main() {
    //create a channel, transitor and receiver
    let (tx, rx) = mpsc::channel(); //mpsc::channel() returns a tuple, sending end--the transmitter and receiving end--the receiver

    //using 'move' to move tx into the closure so the spawned thread owns tx
    //tx has a send method that takes the value we want to send, type is Result
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();  //send returns Result
    });
    /*
    recv() - block the main thread’s execution and wait until a value is sent down the channel, returns Result
    try_recv() - doesn’t block main thread, but will instead return a Result<T, E> immediately, useful if main thread has other work to do while waiting for messages
     */
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}