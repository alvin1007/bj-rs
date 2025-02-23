fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
    use std::fmt::Write;
    let mut stdout = String::new();
    for c in combinations(&(1..=s[0]).collect::<Vec<i64>>(), s[1] as usize) {
        for k in c {
            write!(stdout, "{} ", k).unwrap();
        }
        write!(stdout, "\n").unwrap();
    }
    print!("{stdout}")
}

fn combinations<T: Clone>(vec: &[T], n: usize) -> Vec<Vec<T>> {
    let len = vec.len();
    let mut result = Vec::new();

    if n > len || n == 0 {
        return result;
    }

    if n == 1 {
        return vec.iter().map(|x| vec![x.clone()]).collect();
    }

    for i in 0..=len - n {
        let first = vec[i].clone();
        let sub_combs = combinations(&vec[i + 1..], n - 1);

        for mut sub in sub_combs {
            sub.insert(0, first.clone());
            result.push(sub);
        }
    }

    result
}