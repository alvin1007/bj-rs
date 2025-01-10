fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let t = buffer.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut score = 0;
        let mut suc: i64 = 0;
        for x in input().chars() {
            match x {
                'X' => {
                    suc = 0;
                },
                'O' => {
                    suc += 1;
                    score += suc;
                },
                _ => {},
            }
        }
        println!("{}", score);
    }
}

fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
