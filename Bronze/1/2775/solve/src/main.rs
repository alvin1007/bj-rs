fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let t = buffer.trim().parse::<i64>().unwrap();

    for _ in 0..t {
        let mut buffer = String::new();

        std::io::stdin().read_line(&mut buffer).unwrap();
        let k = buffer.trim().parse::<i64>().unwrap();

        buffer.clear();

        std::io::stdin().read_line(&mut buffer).unwrap();
        let n = buffer.trim().parse::<i64>().unwrap();

        buffer.clear();

        let mut result: Vec<i64> = vec![];

        for i in 1..=n { result.push(i); }

        for _ in 0..k {
            for j in (0..n).rev() {
                result[j as usize] = result[0..=j as usize].iter().sum();
            }
        }

        println!("{}", result[n as usize - 1]);
    }
}
