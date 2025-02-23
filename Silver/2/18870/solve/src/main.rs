fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i64>().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();

    let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    let mut  t = s.clone();

    t.sort();
    t.dedup();

    use std::fmt::Write;
    let mut stdout = String::new();

    use std::cmp::Ordering;

    for i in 0..n {
        let lower_bound = t
        .binary_search_by(|e| match e.cmp(&s[i as usize]) {
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        })
        .unwrap_err();

        write!(stdout, "{} ", lower_bound).unwrap();
    }

    print!("{stdout}")
}
