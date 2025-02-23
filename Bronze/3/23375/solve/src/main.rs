fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let r1 = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let r2 = buf.trim().parse::<i64>().unwrap();

    println!("{} {}", r1[0] + r2, r1[1] + r2);
    println!("{} {}", r1[0] - r2, r1[1] + r2);
    println!("{} {}", r1[0] - r2, r1[1] - r2);
    println!("{} {}", r1[0] + r2, r1[1] - r2);
}
