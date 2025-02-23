fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();

    for i in 1..=n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        let a = s[0].pow(2);
        let b = s[1].pow(2);
        let c = s[2].pow(2);

        if a + b == c || a + c == b || b + c == a {
            println!("Case #{}: YES", i)
        } else {
            println!("Case #{}: NO", i)
        }
    }
}
