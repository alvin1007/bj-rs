fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s = buf.trim().parse::<f64>().unwrap();
    println!("{}", s.powi(2)*3_f64.sqrt()/4.);
}
