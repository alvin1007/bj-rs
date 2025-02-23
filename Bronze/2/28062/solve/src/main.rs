fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();

    let s = buf.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut t1 = vec![];
    let mut t2 = vec![];

    for t in s {
        if t % 2 == 0 {
            t1.push(t);
        } else {
            t2.push(t);
        }
    }

    t2.sort();

    print!("{}", t1.iter().sum::<i64>() + if t2.len() % 2 == 0 { t2.iter().sum() } else if t2.len() > 1 { t2[1..].iter().sum() } else { 0 })
}
