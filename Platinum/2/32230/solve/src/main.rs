fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i32>().unwrap();

    if n < 3 { print!("0"); return; }

    let mut points: Vec<(i64, i64)> = vec![];

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let p = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        points.push((p[0], p[1]));
    }

    points.push(points[0]);

    let mut res = 0;

    for i in 0..points.len() - 1 {
        res += points[i].0*points[i + 1].1 - points[i].1*points[i + 1].0;
    }

    print!("{}", res as f64 * (9. - 3_f64.sqrt()*std::f64::consts::PI) / 54.);
}


