fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let s1 = buffer.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let s2 = buffer.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    if cross((s1[0], s1[1]), (s1[2], s1[3]), (s2[0], s2[1]), (s2[2], s2[3])) {
        print!("1")
    } else {
        print!("0")
    }
}

fn ccw(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> i32 {
    let mut res = p1.0*p2.1 + p2.0*p3.1 + p3.0*p1.1;
    res -= p2.0*p1.1 + p3.0*p2.1 + p1.0*p3.1;
    return if res > 0 { 1 } else { 0 } - if res < 0 { 1 } else { 0 };
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
