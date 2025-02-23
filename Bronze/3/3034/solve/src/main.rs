fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let cfg = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    for _ in 0..cfg[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let n = buffer.trim().parse::<i64>().unwrap();

        if n.pow(2) <= cfg[1].pow(2) + cfg[2].pow(2) {
            println!("DA")
        } else {
            println!("NE")
        }
    }
}
