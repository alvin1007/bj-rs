fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<f64>().unwrap();

    println!("{}", (n/std::f64::consts::PI).sqrt() * 2. * std::f64::consts::PI);
}
