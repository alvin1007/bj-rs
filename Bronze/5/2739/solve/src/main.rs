use std::io;

fn main() {
    let mut input_number = String::new();

    io::stdin().read_line(&mut input_number)
        .expect("Failed to read line");

    let number = match input_number.trim().parse::<i32>() {
        Ok(i) => i,
        Err(_e) => -1,
    };

    for i in 1..10 {
        println!("{} * {} = {}", number, i, number*i);
    }
}
