fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut n = buffer.trim().parse::<i64>().unwrap() - 1;

    let mut cnt = 1;
    let mut i = 1;
    while n > 0 {
        cnt += 1;
        n -= 6*i;
        i += 1;
    }
    print!("{}", cnt);
}
