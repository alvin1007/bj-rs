fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let cfg = buf.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let n = cfg[0];
    let mut k = cfg[1];

    let mut m = vec![];
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        m.push(buf.trim().parse::<i64>().unwrap());
    }

    let mut cnt = 0;

    m.reverse();

    for i in m {
        let temp = k / i;
        cnt += temp;
        k -= i * temp;
        if k == 0 { break; }
    }

    print!("{cnt}");
}
