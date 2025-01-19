fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

    let a = s[0].parse::<i64>().unwrap();
    let b = s[1].parse::<i64>().unwrap();

    let g = gcd(a, b);

    println!("{}", g);
    println!("{}", a*b/g);
}

fn gcd(a: i64, b: i64) -> i64 {
    let r = a % b;
    if r == 0 { return b; }
    return gcd(b, r);
}