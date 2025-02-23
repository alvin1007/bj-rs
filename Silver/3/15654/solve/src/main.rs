fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut t = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
    t.sort();
    use std::fmt::Write;
    let mut stdout = String::new();
    for c in combinations(&t, s[1] as usize) {
        for k in c {
            write!(stdout, "{} ", k).unwrap();
        }
        write!(stdout, "\n").unwrap();
    }
    print!("{stdout}")
}

fn combinations<T: Clone>(vec: &[T], n: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();

    if n == 0 {
        return vec![Vec::new()];
    }

    for i in 0..vec.len() {
        let first = vec[i].clone();
        let mut remaining = vec.to_vec();
        remaining.remove(i);
        let sub_combs = combinations(&remaining, n - 1);

        for mut sub in sub_combs {
            sub.insert(0, first.clone());
            result.push(sub);
        }
    }

    result
}