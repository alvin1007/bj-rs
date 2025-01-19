use std::io;

fn main() {
    let mut input_number = String::new();

    io::stdin().read_line(&mut input_number)
        .expect("Failed to read line");

    let mut x = match input_number.trim().parse::<i32>() {
        Ok(i) => i,
        Err(_e) => -1,
    };

    let mut line_len: i32 = 64;
    let mut result: i32 = 0;

    loop {
        if x == line_len {
            result += 1;
            break;
        } else if x > line_len {
            x -= line_len;
            result += 1;
            continue;
        } else if x < line_len {
            line_len /= 2;
        }
    }

    println!("{}", result);
}