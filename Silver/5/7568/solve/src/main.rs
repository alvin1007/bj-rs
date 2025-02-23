fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let n = next().parse().unwrap();

    let mut s: Vec<(i32, i32)> = vec![];

    for _ in 0..n {
        s.push((next().parse().unwrap(), next().parse().unwrap()));
    }

    for i in 0..n {
        let mut cnt = 1;
        for j in 0..n {
            if j == i { continue; }
            if s[j].0 > s[i].0 && s[j].1 > s[i].1 { cnt += 1; } 
        }
        write!(stdout, "{cnt} ").unwrap()
    }

    print!("{stdout}")
}
