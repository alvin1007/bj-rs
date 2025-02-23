fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let s = buf.trim().parse::<i64>().unwrap();

    use std::fmt::Write;
    let mut stdout = String::new();

    write!(stdout, "{} {}", s * 78 / 100, s * 956 / 1000).unwrap();

    print!("{stdout}")
}