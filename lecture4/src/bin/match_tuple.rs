fn main() {
    let my_triples = [
        (1, 2, 3),
        (10, 20, 30),
        (128, 999, 0),
        (199, 128, 20),
        (128, -128, 5),
    ];

    for triple in my_triples {
        match triple {
            (1, 2, 3) => println!("Got 1, 2, 3"),
            (199, 128, _) => println!("CS 128 Honors!"),
            (128, ..) => println!("We only care that the first item is 128"),
            (x, y, z) => println!("triple adds to {}", x + y + z)
        }
    }
}