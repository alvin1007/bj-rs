fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let a = buffer.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let b = buffer.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let k = buffer.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut ch1 = vec![];
    let mut ch2 = vec![];
    let mut points = vec![];

    for c in a.chunks(2) {
        ch1.push((c[0], c[1]));
    }

    for c in b.chunks(2) {
        ch2.push((c[0], c[1]));
    }

    for c in k.chunks(2) {
        points.push((c[0], c[1]));
    }

    let mut cnt = 0;

    for p in points {
        if !is_inside(&ch1, p) || is_inside(&ch2, p) { cnt += 1; }
    }

    if cnt != 0 {
        println!("{cnt}");
    } else {
        println!("YES");
    }
}

fn is_inside(ch: &Vec<(i64, i64)>, p: (i64, i64)) -> bool {
    let n = ch.len();

    let vl = (ch[n - 1].0 - ch[0].0, ch[n - 1].1 - ch[0].1);
    let vr = (ch[1].0 - ch[0].0, ch[1].1 - ch[0].1);

    let vc = (p.0 - ch[0].0, p.1 - ch[0].1);

    if ccw(vl, vc) > 0 { return false; }
    if ccw(vr, vc) < 0 { return false; }

    let mut l = 1;
    let mut r = n - 1;
    while l + 1 < r {
        let m = (l + r) / 2;
        let vm = (ch[m].0 - ch[0].0, ch[m].1 - ch[0].1);

        if ccw(vm, vc) > 0 { l = m; }
        else { r = m; }
    }

    let v1 = (p.0 - ch[l].0, p.1 - ch[l].1);
    let v2 = (ch[l + 1].0 - p.0, ch[l + 1].1 - p.1);

    return ccw(v1, v2) < 0;
}

fn ccw(p1: (i64, i64), p2: (i64, i64)) -> i32 {
    let res = p1.0 * p2.1 - p1.1 * p2.0;
    return if res > 0 { 1 } else { 0 } - if res < 0 { 1 } else { 0 };
}
