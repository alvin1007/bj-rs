use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer.clear();

    std::io::stdin().read_line(&mut buffer).unwrap();

    let mut a: Vec<i32> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    a.sort();

    buffer.clear();

    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer.clear();

    std::io::stdin().read_line(&mut buffer).unwrap();

    let b: Vec<i32> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut stdout = String::new();

    for &query in &b {
        if a.binary_search(&query).is_ok() {
            write!(stdout, "1\n").unwrap();
        } else {
            write!(stdout, "0\n").unwrap();
        }
    }

    print!("{stdout}")
}