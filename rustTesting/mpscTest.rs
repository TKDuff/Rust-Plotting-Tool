use std::thread;
use std::sync::mpsc;

fn main() {
    //create a channel, transitor and receiver
    let (tx, rx) = mpsc::channel(); //mpsc::channel() returns a tuple, sending end--the transmitter and receiving end--the receiver

    //using 'move' to move tx into the closure so the spawned thread owns tx
    //tx has a send method that takes the value we want to send, type is Result
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}