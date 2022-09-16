fn main() {
    let pairs = vec![
        (10, 0),
        (1, 100),
        (20, 4)
    ];

    for pair in pairs {
        let (a, b) = pair;
        let res: Result<i32, DivisionError> = divide(a, b);

        // println!("divide({}, {}) returns ... {:?}", a, b, res);

        match_divide_result(a, b);

        println!("--------------");
    }
}

#[derive(Debug)]
enum DivisionError {
    DivideByZeroError,
    TruncationError
}

fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZeroError)
    } else if a < b {
        Err(DivisionError::TruncationError)
    } else {
        Ok(a / b)
    }
}

#[allow(dead_code)]
fn match_divide_result(a: i32, b: i32) {
    match divide(a, b) {
        Ok(calculation) => println!("{} / {} = {}", a, b, calculation),
        Err(div_error) => {
            let msg = match div_error {
                DivisionError::DivideByZeroError => "divide by 0",
                DivisionError::TruncationError => "truncation"
            };

            println!("{} / {} errored out: {}", a, b, msg);
        }
    }
}