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