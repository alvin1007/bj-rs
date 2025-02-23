fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    let mut dp = vec![vec![0; s.len()]; s.len()];

    for i in 0..s.len() {
        dp[i][i] = 1;
    }

    for i in 0..s.len()-1 {
        if s[i] == s[i + 1] {
            dp[i][i + 1] = 1;
        }
    }

    for i in (0..s.len()).rev() {
        for j in i..s.len() {
            if i == j { continue; }
            if s[i] == s[j] && dp[i + 1][j - 1] == 1 {
                dp[i][j] = 1;
            }
        }
    }

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i64>().unwrap();

    use std::fmt::Write;
    let mut stdout = String::new();

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let t = buf.split_ascii_whitespace().flat_map(|x|x.parse::<usize>()).collect::<Vec<usize>>();

        writeln!(stdout, "{}", dp[t[0]-1][t[1]-1]).unwrap();
    }

    print!("{stdout}")
}
