fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i64>().unwrap();
    let mut points = vec![];
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let p = buf.split_ascii_whitespace().flat_map(|x|x.parse::<f64>()).collect::<Vec<f64>>();
        points.push((p[0], p[1]));
    }

    shuffle(&mut points);

    let circle = find_min_circle(&points);

    println!("{} {} {}", circle.0.0, circle.0.1, circle.1);
}

fn find_min_circle(v: &Vec<(f64, f64)>) -> ((f64, f64), f64) {
    let mut p = (0., 0.);
    let mut r = 0.;
    let n = v.len();
    let dist = |x: (f64, f64), y: (f64, f64)| {
        return ((x.0 - y.0).powi(2) + (x.1 - y.1).powi(2)).sqrt();
    };

    for i in 0..n {
        if dist(p, v[i]) > r {
            p = v[i];
            r = 0.;
            for j in 0..i {
                if dist(p, v[j]) > r {
                    p = ((v[i].0 + v[j].0) * 0.5, (v[i].1 + v[j].1) * 0.5);
                    r = dist(p, v[i]);
                    for k in 0..j {
                        if dist(p, v[k]) > r {
                            p = circumcenter(v[i], v[j], v[k]);
                            r = dist(p, v[k]);
                        }
                    }
                }
            }
        }
    }

    return (p, r);
}

fn circumcenter(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> (f64, f64) {
    let ab = (b.0 - a.0, b.1 - a.1);
    let ca = (c.0 - a.0, c.1 - a.1);
    let c1 = (ab.0.powi(2) + ab.1.powi(2)) / 2.;
    let c2 = (ca.0.powi(2) + ca.1.powi(2)) / 2.;
    let d = ab.0 * ca.1 - ab.1 * ca.0;
    (a.0 + (c1 * ca.1 - c2 * ab.1) / d, a.1 + (c2 * ab.0 - c1 * ca.0) / d)
}

fn shuffle<T>(vec: &mut [T]) {
    let n: usize = vec.len();
    for i in 0..(n - 1) {
        let j = (rand() as usize) % (n - i) + i;
        vec.swap(i, j);
    }
}

fn rand() -> u64 {
    let n = vec![0];
    let address = &n as *const Vec<i32>;
    address as u64
}