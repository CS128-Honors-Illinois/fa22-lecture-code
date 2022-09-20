fn main() {
    let s = String::from("hello world");

    let hello: &str = &s[0..5];      // same as &s[..5]
    let world: &str = &s[6..s.len()];      // same as &s[6..]
    let hello_world: &str = &s[..]; // same principle as &s, but &s is ACTUALLY &String
    let _hello_world2: &String = &s;

    println!("{}", hello);
    println!("{}", world);
    println!("{}", hello_world);
}   
