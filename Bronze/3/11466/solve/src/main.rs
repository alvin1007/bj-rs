fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    if s[0] > s[1] { s.swap(0, 1); }

    println!("{}", (s[1] as f64 / 3.).max(s[0] as f64 / 2.).min(s[0] as f64));
}
