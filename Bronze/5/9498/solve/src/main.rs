use std::io;

fn main() {
    let mut input_number = String::new();

    io::stdin().read_line(&mut input_number)
        .expect("Failed to read line");

    let number = match input_number.trim().parse::<i32>() {
        Ok(i) => i,
        Err(_e) => -1,
    };

    println!("{}", if number < 60 { "F" } else if number < 70 { "D" } else if number < 80 { "C" } else if number < 90 { "B" } else { "A" })
}
