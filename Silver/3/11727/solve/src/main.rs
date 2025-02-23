fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();

    let mut dp = vec![];

    dp.push(1);
    dp.push(3);

    for i in 2..n {
        dp.push((dp[i - 1] + dp[i - 2] * 2) % 10007);
    }

    println!("{}", dp[n - 1]);
}
