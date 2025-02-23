fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let config = buffer.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut d1: Vec<(i64, i64)> = vec![];
    let mut d2: Vec<(i64, i64)> = vec![];

    for _ in 0..config[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let point = buffer.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        d1.push((point[0], point[1]));
    }

    for _ in 0..config[1] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let point = buffer.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        d2.push((point[0], point[1]));
    }

    let mut ms: Vec<(i64, i64)> = vec![];

    for p in &d1 {
        for q in &d2 {
            ms.push((p.0 + q.0, p.1 + q.1));
        }
    }

    let minkowski_sums = convex_hull(&mut ms);

    use std::io::Write;

    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());

    writeln!(stdout, "{}", minkowski_sums.len()).unwrap();

    for i in minkowski_sums {
        writeln!(stdout, "{} {}", i.0, i.1).unwrap();
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