use std::io;

fn main() {
    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let numbers: Vec<&str> = number.split_whitespace().collect();

    print!("{}",
        (match numbers[0].parse::<i32>() {
            Ok(i) => i,
            Err(_e) => -1
        }.pow(2) +
        match numbers[1].parse::<i32>() {
            Ok(i) => i,
            Err(_e) => -1
        }.pow(2) +
        match numbers[2].parse::<i32>() {
            Ok(i) => i,
            Err(_e) => -1
        }.pow(2) +
        match numbers[3].parse::<i32>() {
            Ok(i) => i,
            Err(_e) => -1
        }.pow(2) +
        match numbers[4].parse::<i32>() {
            Ok(i) => i,
            Err(_e) => -1
        }.pow(2)) % 10
    );
}
