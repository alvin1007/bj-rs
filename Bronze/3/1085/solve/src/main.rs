fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    println!("{}", (n[2] - n[0]).min(n[0]).min((n[3] - n[1]).min(n[1])))
}
