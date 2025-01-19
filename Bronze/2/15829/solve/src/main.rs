// mod 연산 공부..

fn main() {
    let r: i128 = 31;
    let m: i128 = 1234567891;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let l = buffer.trim().parse::<i64>().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let s = buffer.trim().to_string();

    // 각각의 상황에 대해 모듈러 연산을 해야함.
    let mut sum: i128 = 0;
    let mut p: i128 = 1;
    for i in 1..=l {
        let a = (s.as_bytes()[i as usize - 1] - 96) as i128;
        sum += a * p % m;
        p = (p * r) % m;
    }

    print!("{}", sum % m);
}
