use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        tx.send("hello there!".to_string()).unwrap();
        println!("sent message from thread!")
    });

    let message = rx.recv().unwrap();
    println!("Main thread got message: {}", message);
}