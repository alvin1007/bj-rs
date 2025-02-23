fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i64>().unwrap();

    use std::fmt::Write;
    let mut stdout = String::new();

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let s = buf.trim().parse::<i64>().unwrap();
        writeln!(stdout, "{}", "=".repeat(s as usize)).unwrap();   
    }

    print!("{stdout}")
}