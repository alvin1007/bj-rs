fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let p: Vec<f64> = buf.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let d = ((p[0] - p[3]).powi(2) + (p[1] - p[4]).powi(2)).sqrt();

    if (p[2] - p[5]).abs() >= d {
        let r = p[2].min(p[5]);
        println!("{:.3}", (r*r*std::f64::consts::PI * 1000.).round() / 1000.);
        return;
    }

    if p[2] + p[5] <= d {
        println!("0.000");
        return;
    }

    let t1 = 2. * ((d.powi(2) + p[2].powi(2) - p[5].powi(2))/(2.*d*p[2])).acos();
    let t2 = 2. * ((d.powi(2) - p[2].powi(2) + p[5].powi(2))/(2.*d*p[5])).acos();

    let res = ((t1 * p[2].powi(2) + t2 * p[5].powi(2) - p[2].powi(2)*t1.sin() - p[5].powi(2)*t2.sin()) * 500.).round() / 1000.;

    println!("{:.3}", res);
}
