fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<usize>().unwrap();

    let mut stair = vec![];

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        stair.push(buffer.trim().parse::<i64>().unwrap());
    }

    use std::io::Write;

    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());

    let mut s: Vec<i64> = vec![0; n];

    s[0] = stair[0];
    if n == 1 { write!(stdout, "{}", s[0]).unwrap(); return; }

    s[1] = stair[0] + stair[1];
    if n == 2 { write!(stdout, "{}", s[1]).unwrap(); return; }

    s[2] = (stair[0] + stair[2]).max(stair[1] + stair[2]);
    if n == 3 { write!(stdout, "{}", s[2]).unwrap(); return; }

    for i in 3..n {
        s[i] = (s[i - 2] + stair[i]).max(s[i - 3] + stair[i] + stair[i - 1]);
    }

    write!(stdout, "{}", s[n - 1]).unwrap();
}