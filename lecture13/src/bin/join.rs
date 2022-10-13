use std::time::{Duration, Instant};
use std::thread;

fn main() {
    let mut handles = Vec::new();
    for tid in 0..5 {
        let h = thread::spawn(move || {
            println!("Hello from thread {tid}!");
            for _ in 0..3 {
                thread::sleep(Duration::from_millis(1000));
                println!("Hello from thread {tid}!");
            }
        });

        handles.push(h);
    }

    for (idx, h) in handles.into_iter().enumerate() {
        let start = Instant::now();
        h.join().unwrap();
        let duration = Instant::now() - start;
        println!("Joined thread with id={idx} after waiting {} μs", duration.as_micros());
    }
}