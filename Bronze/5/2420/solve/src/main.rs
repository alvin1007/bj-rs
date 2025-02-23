fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let s = buffer.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    println!("{}", (s[0] - s[1]).abs())
}
