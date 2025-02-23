fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i64>().unwrap();
    let mut cnt = 1;
    let mut u = (n % 10) * 10 + (n / 10 + n % 10) % 10;
    while n != u {
        u = (u % 10) * 10 + (u / 10 + u % 10) % 10;
        cnt += 1;
    }
    println!("{cnt}");
}
