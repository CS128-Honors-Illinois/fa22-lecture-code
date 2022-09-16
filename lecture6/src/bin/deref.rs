fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    for elem in v.iter_mut() {
        *elem = *elem + 2;
    }

    println!("{:?}", v);

    for elem in v.iter_mut() {
        *elem *= 2;
    }

    println!("{:?}", v);

    for i in 0..v.len() {
        v[i] += 1;
    }

    println!("{:?}", v);

    let mut total = 0;
    for elem in v.iter() {
        total += elem;
    }

    println!("{}", total);
}