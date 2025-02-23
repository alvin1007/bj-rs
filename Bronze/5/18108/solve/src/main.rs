fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    print!("{}", buffer.trim().parse::<i32>().unwrap() - 543)
}
