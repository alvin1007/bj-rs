fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let mut s = buffer.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut start = 0;
    let mut end = s.len() - 1;

    let mut res1 = 0;
    let mut res2 = s.len() - 1;

    s.sort();

    let mut t = s[start] + s[end];

    while start < end {
        let k = s[start] + s[end];
        if k.abs() < t.abs() {
            t = k;
            res1 = start;
            res2 = end;

            if t == 0 { break; }
        }

        if k < 0 {
            start += 1;
        } else {
            end -= 1;
        }
    }

    print!("{} {}", s[res1], s[res2])
}
