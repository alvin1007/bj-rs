fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<i64>().unwrap();
    
    print!("{}", solve(n));
}

fn solve(n: i64) -> i64 {
    if n <= 1 { return 0; }
    let n1 = solve(n/3) + n%3 + 1;
    let n2 = solve(n/2) + n%2 + 1;
    return n1.min(n2);
}