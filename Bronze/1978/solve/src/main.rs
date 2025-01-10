fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();
    let mut cnt = 0;
    for i in s {
        if is_prime(i.parse::<i64>().unwrap()) { cnt += 1; }
    }
    print!("{}", cnt);
}

fn is_prime(n: i64) -> bool {
    if n == 1 { return false; }
    for i in 2..=((n as f64).sqrt() as i64) {
        if n%i == 0 {
            return false;
        }
    }
    true
}
