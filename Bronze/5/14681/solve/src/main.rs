fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let x = next().parse::<i32>().unwrap();
    let y = next().parse::<i32>().unwrap();

    match (x > 0, y > 0) {
        (true, true) => { write!(stdout, "1").unwrap() }
        (false, true) => { write!(stdout, "2").unwrap() }
        (true, false) => { write!(stdout, "4").unwrap() }
        (false, false) => { write!(stdout, "3").unwrap() }
    }

    print!("{stdout}")
}
