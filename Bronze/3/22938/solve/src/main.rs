fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let r1 = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i128>()).collect::<Vec<i128>>();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let r2 = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i128>()).collect::<Vec<i128>>();

    if (r1[0] - r2[0]).pow(2) + (r1[1] - r2[1]).pow(2) >= (r1[2] + r2[2]).pow(2) {
        println!("NO")
    } else {
        println!("YES")
    }
}
