fn main() {
    demo1();
}

fn demo1() {
    let mut food: Vec<String> = vec!["pizza".to_string(), "ice cream".to_string(), "bread".to_string()];

    let pizza = &mut food[0];
    pizza.push_str("!!!");
    println!("{}", food[0]);

    let ice_cream: &String = &food[1];
    let mut s = "I like ".to_string();
    s.push_str(ice_cream);
    println!("{}", s);

    println!("--------------------------------");

    for f in food.iter() {
        println!("{}", f);
    }

    println!("--------------------------------");

    for f in food.iter_mut() {
        *f = format!("yummy {}", f);
        println!("{}", f);
    }

    println!("--------------------------------");

    for f in food.into_iter() {
        // we have ownership of each element in the vector now...
        println!("{}", f);
    }

    println!("--------------------------------");

    /* COMPILATION ERROR
     * borrow of moved value: `food`
     * vector.rs(line 27): `food` moved due to this method call
     * vector.rs(6, 9): move occurs because `food` has type `Vec<String>`, 
     *  which does not implement the `Copy` trait
     */
    // for f in food.iter() {
    //     println!("{}", f);
    // }
}
