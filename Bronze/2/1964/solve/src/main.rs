fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i64>().unwrap();
    println!("{}",(5*n + (n - 1)*(3*n - 2)/2) % 45678 )
}