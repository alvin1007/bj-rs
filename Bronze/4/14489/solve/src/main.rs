fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let t = buf.trim().parse::<i64>().unwrap();
    print!("{}", if s[0] + s[1] >= 2*t { s[0] + s[1] - 2*t } else { s[0] + s[1] })
}
