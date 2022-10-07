use std::sync::mpsc;
use std::thread;

fn main() {
    let chunk_size = 10_000;
    let num_threads = 10;

    let max_data = chunk_size * num_threads;
    let data = (0..max_data).collect::<Vec<usize>>();

    let (tx, rx) = mpsc::channel();

    for i in 0..num_threads {
        // GET SUB-VECTORS OF SIZE 10,000
        let start = i * chunk_size;
        let end_excl = start + chunk_size;
        let owned_subvec = data[start..end_excl].to_vec();

        let tx_clone = tx.clone();

        thread::spawn(move || {
            let min = owned_subvec[0];
            let max = owned_subvec[owned_subvec.len() - 1];

            let sub_vec_sum: usize = owned_subvec.into_iter().sum();
            println!("Subvec sum from {min} to {max} is {sub_vec_sum}");
            tx_clone.send(sub_vec_sum).unwrap();
        });
    }

    drop(tx);
    let mut total = 0;
    while let Ok(value) = rx.recv() {
        println!("Receiver got {value}!");
        total += value;
    }

    assert_eq!(total, data.into_iter().sum());
    println!("Total sum is: {}", total);
}