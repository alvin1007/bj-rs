fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let wh = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    println!("{}", wh[0] as f64 + wh[1] as f64 - ((wh[0].pow(2) + wh[1].pow(2)) as f64).sqrt());
}
