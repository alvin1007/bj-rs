fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let t = buf.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let s = buf
            .split_ascii_whitespace()
            .flat_map(|x|x.parse::<i32>())
            .collect::<Vec<i32>>();

        println!("{} {}", s[1] * 2 - s[0], s[0] - s[1])
    }
}