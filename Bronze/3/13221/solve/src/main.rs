fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i64>().unwrap();

    let mut min = 1000;
    let mut res = (0, 0);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let t = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        let k =  (t[0] - s[0]).abs() + (t[1] - s[1]).abs();
        if k < min {
            min = k;
            res = (t[0], t[1]);
        }
    }

    println!("{} {}", res.0, res.1);
}
