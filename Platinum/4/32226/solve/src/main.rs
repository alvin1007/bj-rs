fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s: Vec<i64> = buf.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let a = s[0];
    let b = s[1];
    let c = s[2];

    let angle_a = ((b.pow(2) + c.pow(2) - a.pow(2)) as f64 / (2 * b * c) as f64).acos();
    let angle_b = ((a.pow(2) + c.pow(2) - b.pow(2)) as f64 / (2 * a * c) as f64).acos();
    let angle_c = ((b.pow(2) + a.pow(2) - c.pow(2)) as f64 / (2 * a * b) as f64).acos();

    if angle_a >= std::f64::consts::FRAC_PI_2 {
        if b + c <= a {
            println!("0")
        } else {
            println!("{}", intersect(b, c, a))
        }
        return;
    }

    if angle_b >= std::f64::consts::FRAC_PI_2 {
        if a + c <= b {
            println!("0")
        } else {
            println!("{}", intersect(a, c, b))
        }
        return;
    }

    if angle_c >= std::f64::consts::FRAC_PI_2 {
        if b + a <= c {
            println!("0")
        } else {
            println!("{}", intersect(b, a, c))
        }
        return;
    }

    let union = (((a + b + c)*(b + c - a)*(a + b - c)*(a + c - b)) as f64).sqrt() + std::f64::consts::FRAC_PI_2 * (a.pow(2) + b.pow(2) + c.pow(2)) as f64;

    println!("{}", union - std::f64::consts::PI * (a.pow(2) + b.pow(2) + c.pow(2)) as f64 + (intersect(a, b, c) + intersect(b, c, a) + intersect(a, c, b)))

}

fn intersect(a: i64, b: i64, n: i64) -> f64 {
    let t1 = 2. * ((n.pow(2) + a.pow(2) - b.pow(2)) as f64/(2*n*a) as f64).acos();
    let t2 = 2. * ((n.pow(2) - a.pow(2) + b.pow(2)) as f64/(2*n*b) as f64).acos();

    ((t1 * a.pow(2) as f64 + t2 * b.pow(2) as f64 - a.pow(2) as f64 * t1.sin() - b.pow(2) as f64 * t2.sin())) * 0.5
}