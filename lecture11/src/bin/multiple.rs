use std::thread;

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let h = thread::spawn(move || {
            println!("hello from thread {}!", i);
        });

        handles.push(h);
    }

    for h in handles.into_iter() {
        h.join().unwrap();
    }
}