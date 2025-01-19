fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    print!("{}", buffer.split_ascii_whitespace().collect::<Vec<&str>>().len());
}
