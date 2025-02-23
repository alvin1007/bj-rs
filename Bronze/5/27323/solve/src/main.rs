fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let a = buffer.trim().parse::<i64>().unwrap();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let b = buffer.trim().parse::<i64>().unwrap();

    print!("{}", a*b);
}
