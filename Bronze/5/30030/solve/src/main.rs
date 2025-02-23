fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n= buf.trim().parse::<i64>().unwrap();
    println!("{}", n* 10 / 11);
}
