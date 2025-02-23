fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i64>().unwrap();

    for i in 1..=n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        let mut r = s[0];
        let mut cnt = 0.;
        while r != 0 {
            cnt += r.pow(2) as f64;
            r *= s[1];
            cnt += r.pow(2) as f64;
            r /= s[2];
        }
        println!("Case #{}: {}", i, std::f64::consts::PI * cnt);
    }
}
