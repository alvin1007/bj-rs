fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut points: Vec<(i64, i64, i64)> = vec![];

    if n[0] < 3 { println!("0"); return; }

    for i in 0..n[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let s = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        points.push((s[0], s[1], i));
    }
    
    let mut cnt = 0;

    loop {
        if points.len() < 3 { break; }
        let mut k = 0;
        for i in 0..points.len()-2 {
            if ccw((points[i].0, points[i].1), (points[i+1].0, points[i+1].1), (points[i+2].0, points[i+2].1)) != 0 {
                break;
            }
            k += 1;
        }
        if k == points.len() - 2 { break; }
        let t = convex_hull(&mut points);
        if is_inside(&t, (n[1], n[2])) {
            points.retain(|x| !t.contains(&x));
            cnt += 1;
        } else {
            break;
        }
    }

    print!("{cnt}");
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

fn convex_hull(ps: &mut Vec<(i64, i64, i64)>) -> Vec<(i64, i64, i64)> {
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
            let dir = ccw((pivot.0, pivot.1), (x.0, x.1), (y.0, y.1));
            if dir != 0 { return 0_i32.cmp(&dir); }
            return dis((pivot.0, pivot.1), (x.0, x.1)).cmp(&dis((pivot.0, pivot.1), (y.0, y.1)));
        }
    );

    let mut stk: Vec<(i64, i64, i64)> = vec![];
    for p in ps {
        while stk.len() >= 2 && (ccw((stk[stk.len() - 2].0, stk[stk.len() - 2].1), (stk[stk.len() - 1].0, stk[stk.len() - 1].1), (p.0, p.1)) <= 0) { stk.pop(); }
        stk.push(*p);
    }
    return stk;
}

fn ccw2(p1: (i64, i64), p2: (i64, i64)) -> i32 {
    let res = p1.0 * p2.1 - p1.1 * p2.0;
    return if res > 0 { 1 } else { 0 } - if res < 0 { 1 } else { 0 };
}

fn is_inside(ch: &Vec<(i64, i64, i64)>, p: (i64, i64)) -> bool {
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