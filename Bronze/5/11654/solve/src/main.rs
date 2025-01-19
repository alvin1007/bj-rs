fn main() {
    let mut c = String::new();
    std::io::stdin().read_line(&mut c).unwrap();
    print!("{}", c.trim().as_bytes()[0]);
}
