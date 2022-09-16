fn main() {
    let mut class: String = "CS 128 Honors".to_string();

    class.push_str(" is the best");

    let x: &mut String = &mut class;
    *x = String::from("x");

    println!("{}", class);
}

fn say_hello(name: String) { 
    // say_hello takes ownership of `name` here
    println!("Hello {}!", name);
} // name is dropped here

fn say_hello_borrow(name: &String) { 
    // say_hello gets a reference to a string here
    println!("Hello {}!", name);
} // the original String remains after this function

fn say_hello_borrow2(name: &str) { 
    // say_hello gets a reference to a string here
    println!("Hello {}!", name);
} // the original String remains after this function