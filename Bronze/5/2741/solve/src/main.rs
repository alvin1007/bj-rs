use std::io::{self, Write};

fn main() {
    let mut input_number = String::new();
    io::stdin().read_line(&mut input_number)
        .expect("Failed to read line");
        
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    
    for i in 1..match input_number.trim().parse::<i32>() {
        Ok(i) => i,
        Err(_e) => -1,
    }+1 {
        writeln!(out, "{}", i).unwrap();
    }
}