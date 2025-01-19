use std::io;

fn main() {
    let mut input_number = String::new();

    io::stdin().read_line(&mut input_number)
        .expect("Failed to read line");

    let T = match input_number.trim().parse::<i32>() {
        Ok(i) => i,
        Err(_e) => -1,
    };

    // mCn
    {
        let mut i = 0;

        while i < T {
            let test_case: (u128, u128) = input_case();

            println!("{}", factorial(test_case.1)/(factorial(test_case.0)*factorial(test_case.1 - test_case.0)));
            i += 1;
        }
    }
}

/// (N, M)
fn input_case() -> (u128, u128) {
    let mut input_number = String::new();

    io::stdin().read_line(&mut input_number)
        .expect("Failed to read line");

    let numbers: Vec<&str> = input_number.split_whitespace().collect();

    (
        match numbers[0].parse::<u128>() {
            Ok(i) => i,
            Err(_e) => 0
        },
        match numbers[1].parse::<u128>() {
            Ok(i) => i,
            Err(_e) => 0
        }
    )
}

fn factorial(num: u128) -> u128 {
    (1..=num).product()
}