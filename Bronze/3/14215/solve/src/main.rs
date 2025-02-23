fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    s.sort();

    if s[2] >= s[0] + s[1] {
        s[2] = s[0] + s[1] - 1;
    }

    println!("{}", s[0] + s[1] + s[2]);
}
