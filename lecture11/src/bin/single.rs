use std::thread;

fn main() {
    thread::spawn(|| {
        println!("Hello from another thread!");
    });

    println!("Hello from the main thread!");
}