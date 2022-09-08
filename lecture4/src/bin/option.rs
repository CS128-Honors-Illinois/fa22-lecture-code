fn main() {
    let courses = [124, 128, 173, 225, 361];

    for course in courses {
        let professor: Option<&str> = match course {
            124 => Some("Prof. Challen"),
            128 => Some("Prof. Nowak"),
            173 => Some("Prof Fleck"),
            225 => Some("Prof Evans"),
            _ => None
        };
    
        if professor.is_some() {
            let name = professor.unwrap();
            println!("{} teaches CS {}", name, course);
        } else {
            println!("It is unclear who teaches CS {}", course);
        }

        if professor.is_none() {
            println!("It is unclear who teaches CS {}", course);
        } else {
            println!("{:?} teaches CS {}", professor, course);
        }

        println!("-----------")
    }
}