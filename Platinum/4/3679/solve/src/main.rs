fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let t = buffer.trim().parse::<i64>().unwrap();

    use std::io::Write;

    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());

    for _ in 0..t {
        let mut points: Vec<(i64, i64, i64)> = vec![];

        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let s = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        let mut cnt = 0;
        for i in (1..s.len()-1).step_by(2) {
            points.push((s[i], s[i+1], cnt));
            cnt += 1;
        }

        let index_min = points
        .iter()
        .enumerate()
        .min_by(
            |x, y|
            if x.1.0 != y.1.0 { x.1.0.cmp(&y.1.0) } else { x.1.1.cmp(&y.1.1) }
        ).map(|(index, _)| index).unwrap();
        points.swap(0, index_min);

        let pivot = points[0];
        points[1..].sort_by(
            |x, y|
            {
                let dir = ccw((pivot.0, pivot.1), (x.0, x.1), (y.0, y.1));
                if dir != 0 { return 0_i32.cmp(&dir); }
                return dis((pivot.0, pivot.1), (x.0, x.1)).cmp(&dis((pivot.0, pivot.1), (y.0, y.1)));
            }
        );

        let mut r = points.len() - 1;
        while r > 1 && ccw((points[0].0, points[0].1),(points[r].0, points[r].1), (points[r - 1].0, points[r - 1].1)) == 0 {
            r -= 1;
        }
        points[r..].reverse();

        for p in points {
            write!(stdout, "{} ", p.2).unwrap();
        }
        write!(stdout, "\n").unwrap();
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