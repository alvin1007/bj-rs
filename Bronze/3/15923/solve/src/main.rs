fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    
    let mut ps = vec![];

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let p: Vec<i32> = buf.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        ps.push((p[0], p[1]));
    }

    ps.push(ps[0]);

    let mut sum = 0;

    for i in 0..n {
        sum += (ps[i].0 - ps[i + 1].0).abs() + (ps[i].1 - ps[i + 1].1).abs()
    }

    println!("{sum}")
}
