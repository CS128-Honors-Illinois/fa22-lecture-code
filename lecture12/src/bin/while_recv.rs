use std::time::Duration;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("hello").unwrap();
        thread::sleep(Duration::from_millis(1000));
        tx.send("from").unwrap();
        thread::sleep(Duration::from_millis(1000));
        tx.send("another").unwrap();
        thread::sleep(Duration::from_millis(1000));
        tx.send("thread").unwrap();
    }); // tx goes out of scope here

    // this will block until it receives either Ok or Err
    while let Ok(msg) = rx.recv() {
        println!("Main thread got message: {}", msg);
    }
}