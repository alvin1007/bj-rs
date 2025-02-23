fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<i64>().unwrap();

    print!("{}", (n as f64).sqrt() * 4.);
}