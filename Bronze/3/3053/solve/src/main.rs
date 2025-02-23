fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<i64>().unwrap();

    println!("{:.6}", n.pow(2) as f64 * std::f64::consts::PI);
    println!("{:.6}", (2*n.pow(2)) as f64);
}