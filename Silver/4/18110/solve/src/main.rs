fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i64>().unwrap();

    if n == 0 {
        println!("0");
        return;
    }

    let s =  ((n*3) as f64 / 20.).round() as i64;

    let mut v: Vec<i64> = vec![];

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let k = buffer.trim().parse::<i64>().unwrap();
        v.push(k);
    }

    v.sort();

    print!("{}", (v[(s as usize)..(n as usize - s as usize)].iter().sum::<i64>() as f64 / (n - 2*s) as f64).round());
}
