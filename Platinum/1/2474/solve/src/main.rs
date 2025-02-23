fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i64>().unwrap();

    let mut ls = vec![];

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let l = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        ls.push((l[0], l[1]));
    }

    ls.sort_by(|a, b| a.0.cmp(&b.0));

    let cross = |a: (i64, i64), b: (i64, i64)| {
        return (a.0 < b.0 && a.1 > b.1) || (a.0 > b.0 && a.1 < b.1)
    };

    let mut max = 0;

    for i in 0..ls.len() {
        let mut j = i + 1;
        let mut cnt = 0;
        while j < ls.len() && cross(ls[i], ls[j]) { cnt += 1;  j += 1; }
        max = max.max(cnt);
    }

    println!("{}", max);
}
