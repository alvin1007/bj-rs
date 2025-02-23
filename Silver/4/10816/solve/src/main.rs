fn main() {
    let mut s = vec![0; 20000001];

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.split_ascii_whitespace().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    for i in n  {
        s[(i+10000000) as usize] += 1;
    }

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let t = buffer.split_ascii_whitespace().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());

    use std::io::Write;

    for i in t {
        write!(stdout, "{} ", s[(i + 10000000) as usize]).unwrap();
    }
}
