fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: i64 = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let p: Vec<i64> = buf.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if p[0] == p[1] {
        println!("{}", n.pow(2));
        return;
    }

    let d = (p[0] - p[1]).abs() as f64 / 2.;

    println!("{}", n.pow(2) as f64 - d.powi(2));
}
