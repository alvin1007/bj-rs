fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i32>().unwrap();

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let p1 = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let p2 = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

        if p1[0] as f64 / p1[1] as f64 > p2[0].pow(2) as f64 * std::f64::consts::PI / p2[1] as f64 {
            println!("Slice of pizza")
        } else {
            println!("Whole pizza")
        }
    }
 }
