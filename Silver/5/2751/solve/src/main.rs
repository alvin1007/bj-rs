use std::io::{BufWriter, Write};

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<i64>().unwrap();

    let mut s: Vec<i64> = vec![1000001; 1000000];

    for i in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        s[i as usize] = buffer.trim().parse::<i64>().unwrap();
    }

    s.sort();

    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());

    for a in s {
        if a == 1000001 { break; }
        writeln!(&mut out, "{}", a).unwrap();
    }
}
