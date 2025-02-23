fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let cfg = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    let mut ps1 = vec![];

    for _ in 0..cfg[0] {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let p = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        ps1.push((p[0], p[1]));
    }

    ps1 = convex_hull(&mut ps1);

    let mut ps2 = vec![];

    for _ in 0..cfg[1] {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let p = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        ps2.push((p[0], p[1]));
    }

    ps2 = convex_hull(&mut ps2);

    let mut inside_ps = vec![];

    for p in &ps1 {
        if is_inside(&ps2, *p) { inside_ps.push((p.0 as f64, p.1 as f64)); }
    }

    for p in &ps2 {
        if is_inside(&ps1, *p) { inside_ps.push((p.0 as f64, p.1 as f64)); }
    }

    ps1.push(ps1[0]);
    ps2.push(ps2[0]);

    for i in 0..ps1.len()-1 {
        for j in 0..ps2.len()-1 {
            if cross(ps1[i], ps1[i + 1], ps2[j], ps2[j + 1]) {
                let x = ((ps1[i].0 * ps1[i + 1].1 - ps1[i].1 * ps1[i + 1].0) * (ps2[j].0 - ps2[j + 1].0) - (ps1[i].0 - ps1[i + 1].0) * (ps2[j].0 * ps2[j + 1].1 - ps2[j].1 * ps2[j + 1].0)) as f64 / ((ps1[i].0 - ps1[i + 1].0) * (ps2[j].1 - ps2[j + 1].1) - (ps1[i].1 - ps1[i + 1].1) * (ps2[j].0 - ps2[j + 1].0)) as f64;
                let y = ((ps1[i].0 * ps1[i + 1].1 - ps1[i].1 * ps1[i + 1].0) * (ps2[j].1 - ps2[j + 1].1) - (ps1[i].1 - ps1[i + 1].1) * (ps2[j].0 * ps2[j + 1].1 - ps2[j].1 * ps2[j + 1].0)) as f64 / ((ps1[i].0 - ps1[i + 1].0) * (ps2[j].1 - ps2[j + 1].1) - (ps1[i].1 - ps1[i + 1].1) * (ps2[j].0 - ps2[j + 1].0)) as f64;
                inside_ps.push((x, y));
            }
        }
    }

    if inside_ps.len() == 0 { println!("0.0"); return; }

    let index_min = inside_ps
    .iter()
    .enumerate()
    .min_by(|x, y| {
        x.1.0
            .partial_cmp(&y.1.0)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| x.1.1.partial_cmp(&y.1.1).unwrap_or(std::cmp::Ordering::Equal))
    })
    .map(|(index, _)| index)
    .unwrap();

    inside_ps.swap(0, index_min);

    let pivot = inside_ps[0];
    inside_ps[1..].sort_by(|x, y| {
        let dir = fccw3(pivot, *x, *y);
        if dir != 0 {
            return 0_i32.cmp(&dir);
        }
        fdis(pivot, *x)
            .partial_cmp(&fdis(pivot, *y))
            .unwrap_or(std::cmp::Ordering::Equal) 
    });
    
    println!("{}", polygon_area(inside_ps));
}

fn polygon_area(points: Vec<(f64, f64)>) -> f64 {
    let n = points.len();
    if n < 3 {
        return 0.0;
    }

    let mut sum = 0.0;
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];
        sum += x1 * y2 - x2 * y1;
    }

    (sum.abs()) * 0.5
}

fn dis(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    let dx = p2.0 - p1.0; let dy = p2.1 - p1.1;
    return dx*dx + dy*dy;
}

fn fdis(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p2.0 - p1.0; let dy = p2.1 - p1.1;
    return dx*dx + dy*dy;
}

fn convex_hull(ps: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let index_min = ps
        .iter()
        .enumerate()
        .min_by(
            |x, y|
            if x.1.0 != y.1.0 { x.1.0.cmp(&y.1.0) } else { x.1.1.cmp(&y.1.1) }
        ).map(|(index, _)| index).unwrap();
    ps.swap(0, index_min);

    let pivot = ps[0];
    ps[1..].sort_by(
        |x, y|
        {
            let dir = ccw3(pivot, *x, *y);
            if dir != 0 { return 0_i32.cmp(&dir); }
            return dis(pivot, *x).cmp(&dis(pivot, *y));
        }
    );

    let mut stk: Vec<(i64, i64)> = vec![];
    for p in ps {
        while stk.len() >= 2 && (ccw3(stk[stk.len() - 2], stk[stk.len() - 1], *p) <= 0) { stk.pop(); }
        stk.push(*p);
    }
    return stk;
}

fn is_inside(ch: &Vec<(i64, i64)>, p: (i64, i64)) -> bool {
    let n = ch.len();

    let vl = (ch[n - 1].0 - ch[0].0, ch[n - 1].1 - ch[0].1);
    let vr = (ch[1].0 - ch[0].0, ch[1].1 - ch[0].1);

    let vc = (p.0 - ch[0].0, p.1 - ch[0].1);

    if ccw2(vl, vc) > 0 { return false; }
    if ccw2(vr, vc) < 0 { return false; }

    let mut l = 1;
    let mut r = n - 1;
    while l + 1 < r {
        let m = (l + r) / 2;
        let vm = (ch[m].0 - ch[0].0, ch[m].1 - ch[0].1);

        if ccw2(vm, vc) > 0 { l = m; }
        else { r = m; }
    }

    let v1 = (p.0 - ch[l].0, p.1 - ch[l].1);
    let v2 = (ch[l + 1].0 - p.0, ch[l + 1].1 - p.1);

    return ccw2(v1, v2) < 0;
}

fn cross(mut p1: (i64, i64), mut p2: (i64, i64), mut p3: (i64, i64), mut p4: (i64, i64)) -> bool {
    let ab = ccw3(p1, p2, p3) * ccw3(p1, p2, p4);
    let cd = ccw3(p3, p4, p1) * ccw3(p3, p4, p2);
    let compare = |x: (i64, i64), y: (i64, i64)| if x.0 != y.0 { x.0 > y.0 } else { x.1 > y.1 };
    if ab == 0 && cd == 0 {
        if compare(p1, p2) { std::mem::swap(&mut p1, &mut p2); }
        if compare(p3, p4) { std::mem::swap(&mut p3, &mut p4); }
        return !(compare(p3, p2) || compare(p1, p4))
    }
    return ab <= 0 && cd <= 0;
}

fn ccw2(p1: (i64, i64), p2: (i64, i64)) -> i32 {
    let res = p1.0 * p2.1 - p1.1 * p2.0;
    return if res > 0 { 1 } else { 0 } - if res < 0 { 1 } else { 0 };
}

fn ccw3(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> i32 {
    let res = (p1.0*p2.1 + p2.0*p3.1 + p3.0*p1.1) - (p2.0*p1.1 + p3.0*p2.1 + p1.0*p3.1);
    return if res > 0 { 1 } else { 0 } - if res < 0 { 1 } else { 0 };
}

fn fccw3(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> i32 {
    let res = (p1.0*p2.1 + p2.0*p3.1 + p3.0*p1.1) - (p2.0*p1.1 + p3.0*p2.1 + p1.0*p3.1);
    return if res > 0. { 1 } else { 0 } - if res < 0. { 1 } else { 0 };
}