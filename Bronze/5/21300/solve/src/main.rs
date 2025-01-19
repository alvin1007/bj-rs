fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    println!("{}", buffer.split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .flat_map(|x| x.parse::<i64>())
        .collect::<Vec<i64>>()
        .iter()
        .map(|x| 5*x)
        .sum::<i64>());
}
