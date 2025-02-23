fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let t = buf.trim().parse::<f64>().unwrap();
    println!("{}", (t.powi(2) / 4.).round() as i64);
}
