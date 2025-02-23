fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let t = buf.trim().parse::<i64>().unwrap();
 
    for _ in 0..t {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<i64>().unwrap();

        let mut dp = vec![];
        dp.push(1);
        dp.push(2);
        dp.push(4);

        for i in 3..n {
            dp.push(dp[i as usize - 3] + dp[i as usize - 2] + dp[i as usize - 1]);
        }

        println!("{}", dp[n as usize - 1]);
   }
}
