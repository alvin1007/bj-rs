fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();

    let mut dp: Vec<i64> = vec![0; 1000];

    dp[0] = 1;
    dp[1] = 2;

    for i in 2..n {
        dp[i] = dp[i - 1] + dp[i - 2];
        dp[i] %= 10007;
    }

    println!("{}", dp[n-1])
}
