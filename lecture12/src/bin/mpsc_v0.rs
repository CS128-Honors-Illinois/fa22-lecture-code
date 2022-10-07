use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for thread_idx in 0..10 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for i in 0..10 {
                let msg = format!("message {i} from thread {thread_idx}");
                tx_clone.send(msg).unwrap();
            }
        });
    }

    drop(tx);
    while let Ok(msg) = rx.recv() {
        println!("Main thread got message: {msg:?}");
    }
}