fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    let k = s[1].pow(2) + s[2].pow(2);

    for _ in 0..s[0] {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let t = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        if t[0].pow(2) <= k {
            println!("YES")
        } else {
            println!("NO")
        }
    }
}
