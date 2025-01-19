fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();

    std::io::stdin().read_line(&mut buffer).unwrap();

    let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>().iter().flat_map(|x| x.parse::<f64>()).collect::<Vec<f64>>();

    let mut max: f64 = 0.;

    for i in &s {
        if max < *i { max = *i; }
    }

    let mut res: f64 = 0.;

    for i in &s {
        res += *i;
    }

    println!("{}", (100. * res)/(s.len() as f64 * max));
}
