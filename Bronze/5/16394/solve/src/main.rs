fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    print!("{}", buf.trim().parse::<i32>().unwrap() - 1946);
}
