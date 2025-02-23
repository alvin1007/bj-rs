fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i32>().unwrap();

    use std::io::Write;

    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let p = buf.split_ascii_whitespace().flat_map(|x|x.parse::<f64>()).collect::<Vec<f64>>();

        writeln!(stdout, "{}", 2. * (((p[0] - p[2]).powi(2) + (p[1] - p[3]).powi(2)).sqrt() / (2. * (p[1] * p[3]).sqrt())).asinh()).unwrap();
    }
}
