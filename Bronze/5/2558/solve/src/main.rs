fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    write!(stdout, "{}", next().parse::<i32>().unwrap()+ next().parse::<i32>().unwrap()).unwrap();
    print!("{stdout}")
}
