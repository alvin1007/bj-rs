fn main() {
    let mut mem: Vec<i64> = vec![2];    

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    
    for i in 2..=n[1] {

        let mut is_prime = true;
        for s in 2..(i as f64).sqrt() as i64 + 1 {
            if i % s == 0 { is_prime = false; break; }
        }

        if is_prime {
            if i >= n[0] { println!("{}", i) }
            mem.push(i);
        }
    }
}
