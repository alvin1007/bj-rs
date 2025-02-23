fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let t = buf.trim().parse::<i64>().unwrap();

    for i in 1..=t {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<i64>().unwrap();

        let mut xmax: f64 = -1000.;
        let mut xmin: f64 = 1000.;
        let mut ymax: f64 = -1000.;
        let mut ymin: f64 = 1000.;

        for _ in 0..n {
            buf.clear();
            std::io::stdin().read_line(&mut buf).unwrap();
            let p = buf.split_ascii_whitespace().flat_map(|x|x.parse::<f64>()).collect::<Vec<f64>>();

            xmax = xmax.max(p[0]);
            xmin = xmin.min(p[0]);
            ymax = ymax.max(p[1]);
            ymin = ymin.min(p[1]);
        }

        println!("Case {}: Area {}, Perimeter {}", i, (xmax - xmin)*(ymax - ymin), (xmax - xmin) * 2. + (ymax - ymin) * 2.);
    }
}
