fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let t = buf.trim().parse::<i128>().unwrap();
 
    for _ in 0..t {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<i64>().unwrap();

        let mut dp: Vec<i64> = vec![];
        dp.push(1);
        dp.push(1);
        dp.push(1);
        dp.push(2);
        dp.push(2);

        for i in 5..n {
            dp.push(dp[i as usize - 1] + dp[i as usize - 5]);
        }

        println!("{}", dp[n as usize - 1]);
   }
}

