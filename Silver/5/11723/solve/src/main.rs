use std::io::{Write, BufWriter};

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<i64>().unwrap();

    let mut res: Vec<bool> = vec![false; 20];

    let stdout = std::io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    for _ in 0..n {
        buffer.clear();

        std::io::stdin().read_line(&mut buffer).unwrap();
        let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

        match s[0] {
            "add" => { let x = (s[1].parse::<i16>().unwrap() - 1) as usize; if !res[x] { res[x] = true; } },
            "remove" => { let x = (s[1].parse::<i16>().unwrap() - 1) as usize; if res[x] { res[x] = false; } },
            "check" => { let x = (s[1].parse::<i16>().unwrap() - 1) as usize; if res[x] { writeln!(stdout,"1").unwrap(); } else { writeln!(stdout,"0").unwrap(); } },
            "toggle" => { let x = (s[1].parse::<i16>().unwrap() - 1) as usize; if !res[x] { res[x] = true; } else { res[x] = false; } },
            "all" => { res = vec![true; 20]; },
            "empty" => { res = vec![false; 20]; },
            _ => {},
        }
    }
}
