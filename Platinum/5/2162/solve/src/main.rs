fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();

    let mut ls = vec![];
    let mut parent = [0; 3000];
    let mut cnt = [0; 3000];

    for i in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let p = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        ls.push((p[0], p[1], p[2], p[3]));
        parent[i] = i;
    }

    for i in 0..n {
        for j in 0..i {
            if cross((ls[i].0, ls[i].1), (ls[i].2, ls[i].3), (ls[j].0, ls[j].1), (ls[j].2, ls[j].3)) {
                union_set(i, j, &mut parent);
            }
        }
    }

    let mut group_cnt = 0;
    let mut max = 0;

    for i in 0..n {
        if parent[i] == i {
            group_cnt += 1;
        }
        cnt[get_parent(i, &mut parent)] += 1;
        max = max.max(cnt[get_parent(i, &mut parent)]);
    }

    println!("{}\n{}", group_cnt, max);
}

fn union_set(mut a: usize, mut b: usize, parent: &mut [usize]) {
    a = get_parent(a, parent);
    b = get_parent(b, parent);
    if a < b { parent[b] = a; }
    else { parent[a] = b; }
}

fn get_parent(a: usize, parent: &mut [usize]) -> usize {
    if a == parent[a] { return parent[a] }
    parent[a] = get_parent(parent[a], parent);
    return parent[a];
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

fn ccw(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> i32 {
    let mut res = p1.0*p2.1 + p2.0*p3.1 + p3.0*p1.1;
    res -= p2.0*p1.1 + p3.0*p2.1 + p1.0*p3.1;
    return if res > 0 { 1 } else { 0 } - if res < 0 { 1 } else { 0 };
}
