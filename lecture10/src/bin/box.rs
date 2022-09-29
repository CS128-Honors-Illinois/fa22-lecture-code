fn main() {
    let mut b: Box<String> = Box::new(String::from("CS 128 Honors"));

    println!("{}", b);

    b.push_str(" is the best");

    println!("{}", b);

    *b = String::from("hello there!");

    println!("{}", b);
}
