use std::thread;

fn main() {
    let mut handles = Vec::new();
    let data = String::from("Here's a random message");

    for i in 0..10 {
        let data_clone = data.clone();
        
        let h = thread::spawn(move || {
            let msg = format!("{} -- from thread {}", data_clone, i);
            println!("{msg}");
        });

        handles.push(h);
    }

    for h in handles.into_iter() {
        h.join().unwrap();
    }
}