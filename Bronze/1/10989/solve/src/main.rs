use std::io::{self, Write};

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let t = buffer.trim().parse::<i64>().unwrap();

    let mut res = [0; 10000];

    for _ in 0..t {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        res[buffer.trim().parse::<i64>().unwrap() as usize - 1] += 1;
    }

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    
    for i in 0..10000 {
        for _ in 0..res[i as usize] {
            writeln!(out, "{}", i + 1).unwrap();
        }
    }
}
