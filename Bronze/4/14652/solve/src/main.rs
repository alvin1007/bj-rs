fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let _ = next().parse::<i32>().unwrap();
    let m = next().parse::<i32>().unwrap();
    let k = next().parse::<i32>().unwrap();

    write!(stdout, "{} {}", k / m, k % m).unwrap();

    print!("{stdout}")
}
