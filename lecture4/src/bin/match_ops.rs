fn main() {
    let course_nums: [u32; 10] = [124, 128, 233, 361, 225, 341, 440, 10, 523, 699];

    for course in course_nums {
        let msg = match course {
            0 ..= 99 => "INVALID",
            128 | 225 | 341 => "Teaches C or C++",
            100 ..= 199 => "100 Level",
            level @ 200 ..= 399 => {
                if level >= 300 {
                    "300 Level"
                } else {
                    "200 Level"
                }
            },
            400 ..= 499 => "Upper level electives",
            500 ..= 599 => "Graduate level",
            _ => "INVALID"
        };

        println!("CS {} - {}", course, msg);
    }
}