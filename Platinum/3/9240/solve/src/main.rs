fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i64>().unwrap();

    let mut points: Vec<(i64, i64)> = vec![];

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let ps = buf.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        points.push((ps[0], ps[1]));
    }

    let convex_hull_points = convex_hull(&mut points);

    let dist = rotating_calipers(convex_hull_points);

    println!("{}", (dist as f64).sqrt());
}

fn ccw(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> i32 {
    let mut res = p1.0*p2.1 + p2.0*p3.1 + p3.0*p1.1;
    res -= p2.0*p1.1 + p3.0*p2.1 + p1.0*p3.1;
    return if res > 0 { 1 } else { 0 } - if res < 0 { 1 } else { 0 };
}

fn dis(p1: (i64, i64), p2: (i64, i64)) -> i64 {
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
            let dir = ccw(pivot, *x, *y);
            if dir != 0 { return 0_i32.cmp(&dir); }
            return dis(pivot, *x).cmp(&dis(pivot, *y));
        }
    );

    let mut stk: Vec<(i64, i64)> = vec![];
    for p in ps {
        while stk.len() >= 2 && (ccw(stk[stk.len() - 2], stk[stk.len() - 1], *p) <= 0) { stk.pop(); }
        stk.push(*p);
    }
    return stk;
}

fn rotating_calipers(ps: Vec<(i64, i64)>) -> i64 {
    let n = ps.len();

    let mut left = 0;
    let mut right = 0;

    for i in 0..n {
        if ps[i].0 < ps[left].0 { left = i; }
        if ps[i].0 > ps[right].0 { right = i; }
    }

    let mut dist = dis(ps[left], ps[right]);

    for _ in 0..n {
        let mut prev = (0, 0);
        prev.0 = ps[(left + 1) % n].0 - ps[left].0;
        prev.1 = ps[(left + 1) % n].1 - ps[left].1;

        let mut next = (0, 0);
        next.0 = ps[right].0 - ps[(right + 1) % n].0;
        next.1 = ps[right].1 - ps[(right + 1) % n].1;

        if ccw((0, 0), prev, next) > 0 {
            left = (left + 1) % n;
        } else {
            right = (right + 1) % n;
        }

        dist = dist.max(dis(ps[left], ps[right]));
    }

    return dist;
}