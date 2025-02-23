fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i64>().unwrap();

    let mut cnt = 0;

    use std::fmt::Write;
    let mut stdout = String::new();

    for _ in 0..n-1 {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        cnt += buf.trim().parse::<i64>().unwrap() - 1;
    }

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    cnt += buf.trim().parse::<i64>().unwrap();

    writeln!(stdout, "{}", cnt).unwrap();

    print!("{stdout}")
}