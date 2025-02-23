fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let ss = buf.trim().parse::<String>().unwrap();
    let s = ss.as_bytes();

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

    let mut ans = vec![0; s.len() + 1];

    for i in 1..=s.len() {
        for j in 1..i+1 {
            if dp[j - 1][i - 1] == 1 {
                if ans[i] == 0 {
                    ans[i] = ans[i - 1] + 1;
                }
                ans[i] = ans[i].min(ans[j - 1] + 1);
            }
        }
        if ans[i] == 0 {
            ans[i] = ans[i - 1] + 1;
        }
    }

    println!("{:?}", ans.last().unwrap())
}
