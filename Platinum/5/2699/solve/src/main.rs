fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let t = buf.trim().parse::<i32>().unwrap();

    use std::fmt::Write;
    let mut stdout = String::new();

    for _ in 0..t {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<i32>().unwrap();

        let mut ps = vec![];

        for _ in 0..(n - 1)/5 + 1 {
            buf.clear();
            std::io::stdin().read_line(&mut buf).unwrap();
            let p = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
            for i in (0..p.len()).step_by(2) {
                ps.push((p[i], p[i + 1]));
            }
        }

        let mut ch = convex_hull(&mut ps);
        ch[1..].reverse();
        
        writeln!(stdout, "{}", ch.len()).unwrap();

        for c in ch {
            writeln!(stdout, "{} {}", c.0, c.1).unwrap();
        }
    }

    print!("{stdout}");
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
            if x.1.1 != y.1.1 { y.1.1.cmp(&x.1.1) } else { x.1.0.cmp(&y.1.0) }
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