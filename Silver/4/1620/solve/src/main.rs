use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let cfg = buf.split_ascii_whitespace().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut p1: HashMap<usize, String> = Default::default();
    let mut p2: HashMap<String, usize> = Default::default();

    for i in 0..cfg[0] {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        p1.insert(i, buf.trim().to_string());
        p2.insert(buf.trim().to_string(), i);
    }

    use std::fmt::Write;
    let mut stdout = String::new();

    for _ in 0..cfg[1] {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let str = buf.trim();
        match str.parse::<usize>() {
            Ok(i) => { writeln!(stdout, "{}", p1.get(&(i - 1)).unwrap()).unwrap(); },
            Err(_) => { writeln!(stdout, "{}", p2.get(&str.to_string()).unwrap() + 1).unwrap(); }
        }
    }

    print!("{stdout}")
}
