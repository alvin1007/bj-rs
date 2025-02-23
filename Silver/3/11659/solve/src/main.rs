fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let cfg = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    
    let n = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    use std::fmt::Write;
    let mut stdout = String::new();

    let mut sum: Vec<i64> = vec![];
    sum.push(0);

    for i in 0..cfg[0] {
        sum.push(sum[i as usize] + n[i as usize]);
    }

    for _ in 0..cfg[1] {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let t = buf.split_ascii_whitespace().flat_map(|x|x.parse::<usize>()).collect::<Vec<usize>>();
        writeln!(stdout, "{}", sum[t[1]] - sum[t[0] - 1]).unwrap();
    }

    print!("{stdout}")
}
