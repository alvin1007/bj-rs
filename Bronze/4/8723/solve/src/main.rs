fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    if n[0] == n[1] && n[1] == n[2] {
        print!("2");
        return;
    }

    if n[0].pow(2) == n[1].pow(2) + n[2].pow(2) || n[1].pow(2) == n[0].pow(2) + n[2].pow(2) || n[2].pow(2) == n[1].pow(2) + n[0].pow(2) {
        print!("1");
        return;
    }

    print!("0");
}
