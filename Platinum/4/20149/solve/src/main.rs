fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut p12 = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut p34 = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    let compare = |x: (i64, i64), y: (i64, i64)| if x.0 != y.0 { x.0 > y.0 } else { x.1 > y.1 };

    if cross((p12[0], p12[1]), (p12[2], p12[3]), (p34[0], p34[1]), (p34[2], p34[3])) {
        if (p12[3] - p12[1]) * (p34[2] - p34[0]) == (p12[2] - p12[0]) * (p34[3] - p34[1]) {

            if compare((p12[0], p12[1]), (p12[2], p12[3])) { p12.swap(0, 2); p12.swap(1, 3); }
            if compare((p34[0], p34[1]), (p34[2], p34[3])) { p34.swap(0, 2); p34.swap(1, 3); }

            if (p34[0], p34[1]) == (p12[2], p12[3]) || (p34[2], p34[3]) == (p12[0], p12[1]) {
                    println!("1");
                    if (p34[0], p34[1]) == (p12[2], p12[3]) {
                        println!("{} {}", p34[0], p34[1]);   
                    } else {
                        println!("{} {}", p34[2], p34[3]);
                    }
            } else {
                println!("1");
            }
        } else {
            println!("1");
            let x = ((p12[0] * p12[3] - p12[1] * p12[2]) * (p34[0] - p34[2]) - (p12[0] - p12[2]) * (p34[0] * p34[3] - p34[1] * p34[2])) as f64 / ((p12[0] - p12[2]) * (p34[1] - p34[3]) - (p12[1] - p12[3]) * (p34[0] - p34[2])) as f64;
            let y = ((p12[0] * p12[3] - p12[1] * p12[2]) * (p34[1] - p34[3]) - (p12[1] - p12[3]) * (p34[0] * p34[3] - p34[1] * p34[2])) as f64 / ((p12[0] - p12[2]) * (p34[1] - p34[3]) - (p12[1] - p12[3]) * (p34[0] - p34[2])) as f64;
            println!("{} {}", x, y);
        }
    } else {
        println!("0");
    }
}

fn cross(mut p1: (i64, i64), mut p2: (i64, i64), mut p3: (i64, i64), mut p4: (i64, i64)) -> bool {
    let ab = ccw(p1, p2, p3) * ccw(p1, p2, p4);
    let cd = ccw(p3, p4, p1) * ccw(p3, p4, p2);
    let compare = |x: (i64, i64), y: (i64, i64)| if x.0 != y.0 { x.0 > y.0 } else { x.1 > y.1 };
    if ab == 0 && cd == 0 {
        if compare(p1, p2) { std::mem::swap(&mut p1, &mut p2); }
        if compare(p3, p4) { std::mem::swap(&mut p3, &mut p4); }
        return !(compare(p3, p2) || compare(p1, p4))
    }
    return ab <= 0 && cd <= 0;
}

fn ccw(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> i32 {
    let mut res = p1.0*p2.1 + p2.0*p3.1 + p3.0*p1.1;
    res -= p2.0*p1.1 + p3.0*p2.1 + p1.0*p3.1;
    return if res > 0 { 1 } else { 0 } - if res < 0 { 1 } else { 0 };
}
