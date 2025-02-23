fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let cfg = buffer.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    let mut ps: Vec<(i64, i64)> = vec![];

    for _ in 0..cfg[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let p = buffer.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        ps.push((p[0], p[1]));
    }

    let p = convex_hull(&mut ps);

    let mut result = vec![];
    let mut current = vec![];

    combinations(&p, cfg[1] as usize, 0, &mut current, &mut result);

    let mut max = 0;

    for c in result.iter_mut() {
        c.push(c[0]);

        let mut res = 0;

        for i in 0..c.len()-1 {
            res += c[i].0*c[i+1].1 - c[i].1*c[i+1].0
        }

        if res > max { max = res; }
    }

    println!("{}", max as f64 / 2.);
}

fn combinations<'a, T>(items: &'a [T], k: usize, start: usize, curr: &mut Vec<&'a T>, result: &mut Vec<Vec<&'a T>>) {
    if curr.len() == k {
        result.push(curr.clone());
        return;
    }

    for i in start..items.len() {
        curr.push(&items[i]);
        combinations(items, k, i + 1, curr, result);
        curr.pop();
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