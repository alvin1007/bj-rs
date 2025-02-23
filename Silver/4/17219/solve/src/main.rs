fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let t = buffer.split_ascii_whitespace().flat_map(|x|x.parse::<i32>()).collect::<Vec<i32>>();

    let mut h: std::collections::HashMap<String, String> = Default::default();

    for _ in 0..t[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();
        h.insert(s[0].to_string(), s[1].to_string());
    }

    use std::io::Write;

    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());

    for _ in 0..t[1] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let k = buffer.trim();
        writeln!(stdout, "{}", h.get(&k.to_string()).unwrap()).unwrap();
    }
}