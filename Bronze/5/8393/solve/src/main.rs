fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    print!("{}", (1..=buffer.trim().parse::<i32>().unwrap()).sum::<i32>())
}
