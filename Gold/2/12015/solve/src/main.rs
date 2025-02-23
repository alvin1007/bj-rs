fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let _ = buf.trim().parse::<i64>().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let a = buf.split_ascii_whitespace().flat_map(|x|x.parse::<usize>()).collect::<Vec<usize>>();

    let mut res = vec![];

    res.push(a[0]);

    for i in 1..a.len() {
        if a[i] > *res.last().unwrap() {
            res.push(a[i]);
        } else {
            let idx = match res.binary_search(&a[i]) {
                Ok(i) | Err(i) => i,
            };
            res[idx] = a[i];
        }
    }

    println!("{}", res.len())
}
