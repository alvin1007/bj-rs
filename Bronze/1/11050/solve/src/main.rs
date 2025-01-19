fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

    let n = s[0].parse::<i64>().unwrap();
    let k = s[1].parse::<i64>().unwrap();
    println!("{}", factorial(n)/(factorial(k)*factorial(n-k)));
}

fn factorial(n: i64) -> i64 {
    (1..=n).product()
}