fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut p: Vec<(i64, i64)> = vec![];

    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let s = buffer.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        p.push((s[0], s[1]));
    }

    p.push(p[0]);

    let mut res = 0;

    for i in 0..p.len()-1 {
        res += p[i].0*p[i+1].1 - p[i].1*p[i+1].0
    }

    println!("{:.1}", ((res as f64 / 2.).abs() * 10.).round() / 10.);
}
