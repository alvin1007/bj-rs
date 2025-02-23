fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i32>().unwrap();
    let mut res: Vec<(i32, String)> = vec![];

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

        res.push((s[0].parse::<i32>().unwrap(), s[1].to_string()));
    }

    res.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());

    use std::io::{BufWriter, Write};

    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());

    for s in &res {
        writeln!(out, "{} {}", s.0, s.1).unwrap();
    }
}

