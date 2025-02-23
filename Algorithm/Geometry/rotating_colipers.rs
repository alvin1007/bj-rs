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