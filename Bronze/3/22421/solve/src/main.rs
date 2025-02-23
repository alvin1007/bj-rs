fn main() {
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let t = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        if t[0] == 0 && t[1] == 0 {
            break;
        }

        let mut min: f64 = 100.;

        for i in 0..=t[0] {
            let k = (((i.pow(2) + (t[0] - i).pow(2)) as f64).sqrt() - t[1] as f64).abs();
            min = min.min(k);
        }

        println!("{}", min);
    }
}
