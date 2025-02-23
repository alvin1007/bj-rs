fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut t = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
    t.sort();
    use std::fmt::Write;
    let mut stdout = String::new();
    for c in combinations(&t, s[1] as usize) {
        for k in c {
            write!(stdout, "{} ", k).unwrap();
        }
        write!(stdout, "\n").unwrap();
    }
    print!("{stdout}")
}