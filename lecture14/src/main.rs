use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let clone = data.clone();
        let h = thread::spawn(move || {
            for _ in 0..20 {
                *clone.lock().unwrap() += 1;
            } // the MutexGuard returned by .lock() goes out of scope
            // Other threads can attempt to lock the mutex now...
        });

        handles.push(h);
    }

    for h in handles.into_iter() {
        h.join().unwrap();
    }

    println!("The final count is {}", data.lock().unwrap());
}
