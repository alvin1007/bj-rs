fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    let mut v: Vec<i64> = vec![];

    for i in 1..=n.len() {
        let mut cnt = 0;

        if i != n.len() {
            let mut right_max_slope = (-1, -1); // (a, b) = b / a
            for j in i+1..=n.len() {
                if right_max_slope.0 == -1 {
                    right_max_slope = (j as i64 - i as i64, n[j - 1] - n[i - 1]);
                    cnt += 1;
                }

                if is_show((i as i64, n[i - 1]), (j as i64, n[j - 1]), right_max_slope) {
                    right_max_slope = (j as i64 - i as i64, n[j - 1] - n[i - 1]);
                    cnt += 1;
                }
            }
        }

        if i != 1 {
            let mut left_max_slope = (-1, -1);
            for j in (1..=i-1).rev() {
                if left_max_slope.0 == -1 {
                    left_max_slope = (-(j as i64) + (i as i64), n[j - 1] - n[i - 1]);
                    cnt += 1;
                }
    
                if is_show((-(i as i64), n[i - 1]), (-(j as i64), n[j - 1]), left_max_slope) {
                    left_max_slope = (-(j as i64) + i as i64, n[j - 1] - n[i - 1]);
                    cnt += 1;
                }
            }
        }

        v.push(cnt);
    }

    print!("{}", v.iter().max().unwrap());
}

fn is_show(a: (i64, i64), b: (i64, i64), max: (i64, i64)) -> bool {
    let slope = (b.0 - a.0, b.1 - a.1);
    max.1 * slope.0 < slope.1 * max.0
}