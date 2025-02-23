fn main() {
    let mut cnt = 1;

    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<i32>().unwrap();

        if n == 0 { break; }

        let mut ps = vec![];

        for _ in 0..n {
            buf.clear();
            std::io::stdin().read_line(&mut buf).unwrap();
            let p = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
            ps.push((p[0], p[1]));
        }

        let mut ch = convex_hull(&mut ps);

        ch.push(ch[0]);

        let mut min_len: f64 = 1e18;
        let cp = |p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)| {
            let res = p1.0*p2.1 + p2.0*p3.1 + p3.0*p1.1 - (p2.0*p1.1 + p3.0*p2.1 + p1.0*p3.1);
            return res;
        };

        for i in 0..ch.len()-1 {
            let mut j = 0;
            let mut max_len: f64 = 0.;
            while j < ch.len() - 1 {
                if i == j || i + 1 == j { j += 1; continue; }
                let cp_ = cp(ch[i], ch[i + 1], ch[j]);
                let dis = (((ch[i].0 - ch[i + 1].0).pow(2) + (ch[i].1 - ch[i + 1].1).pow(2)) as f64).sqrt();
                max_len = max_len.max(cp_ as f64 / dis);
                j += 1;
            }
            min_len = min_len.min(max_len);
        }

        println!("Case {}: {:.2}", cnt, (min_len * 100.).ceil() / 100.);

        cnt += 1;
    }
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