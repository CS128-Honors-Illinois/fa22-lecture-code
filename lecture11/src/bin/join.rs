use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from another thread!");
    });

    // Wait for our thread to finish
    match handle.join() {
        Ok(_) => (), // do nothing if our thread finished successfully
        Err(e) => println!("Thread join error {:?}", e)
    }

    println!("Hello from the main thread!");
}