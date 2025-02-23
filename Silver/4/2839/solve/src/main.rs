fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i64>().unwrap();

    let mut cnt5 = n / 5;

    while (n - cnt5*5) % 3 != 0 {
        cnt5 -= 1;
    }
    if cnt5 < 0 {
        print!("-1");
    } else {
        print!("{}", cnt5 + (n - cnt5*5) / 3);
    }
}
