fn main() {
    let course = get_course_number();

    let professor = match course {
        124 => "Prof. Challen",
        128 => "Prof. Nowak",
        173 => "Prof Fleck",
        225 => "Prof Evans",
        _ => ""
    };

    println!("{} teaches CS {}", professor, course);
}

fn get_course_number() -> i32 {
    128
}