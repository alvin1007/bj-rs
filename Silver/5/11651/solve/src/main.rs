fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i32>().unwrap();
    let mut points: Vec<(i32, i32)> = vec![];

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

        points.push((s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap()));
    }

    points.sort_by(|x, y| if x.1 == y.1 { x.0.partial_cmp(&y.0).unwrap() } else { x.1.partial_cmp(&y.1).unwrap() });

    use std::io::{BufWriter, Write};

    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());

    for s in &points {
        writeln!(out, "{} {}", s.0, s.1).unwrap();
    }
}
