use std::io;

fn main() {
    let mut input_number = String::new();

    io::stdin().read_line(&mut input_number)
        .expect("Failed to read line");

    let number = match input_number.trim().parse::<i32>() {
        Ok(i) => i,
        Err(_e) => -1,
    };

    if (number % 100 != 0 && number % 4 == 0) || (number % 400 == 0) {
        println!("1")
    } else {
        println!("0")
    }
}
