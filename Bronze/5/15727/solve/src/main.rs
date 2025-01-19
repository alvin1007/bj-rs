fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let a = buffer.trim().parse::<i64>().unwrap();
    if a % 5 == 0 { println!("{}", a/5) } else { println!("{}", a/5 + 1) }
}
