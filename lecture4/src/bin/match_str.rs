fn main() {
    let call: String = get_random_call();

    let response = match call.as_str() {
        "ILL" => "INI!",
        "To infinity" => "And beyond!",
        "Hakuna" => "Matata!",
        "Marco" => "Polo!",
        _ => "I don't know how to respond to that"
    };

    println!("{}", response);
}

fn get_random_call() -> String {
    String::from("Marco")
}