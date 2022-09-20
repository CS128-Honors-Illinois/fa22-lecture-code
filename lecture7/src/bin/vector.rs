fn main() {
    let v: Vec<i32> = vec![1, 2, 8, 1, 9, 9];
    
    let my_slice: &[i32] = &v[..3];

    for val in my_slice {
        print!("{} ", val);
    }

    println!("\n-------------------------");

    for val in &v[2..5] {
        print!("{} ", val);
    }

    println!("\n-------------------------");

    for start_idx in 0..(v.len() - 2) {
        let subarray = &v[start_idx..start_idx+3];
        println!("{:?}", subarray);
    }
}