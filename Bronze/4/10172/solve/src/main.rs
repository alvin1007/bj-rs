use std::io;

fn main() {
    let mut input_number = String::new();

    io::stdin().read_line(&mut input_number)
        .expect("Failed to read line");

    let a:i32 = match input_number.split_whitespace().collect::<Vec<&str>>()[0].parse::<i32>() {
        Ok(i) => i,
        Err(_e) => -1,
    };

    let b:i32 = match input_number.split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>() {
        Ok(i) => i,
        Err(_e) => -1,
    };

    print!("{}\n{}\n{}\n{}\n{}", a+b, a-b, a*b, a/b, a%b)
}
