fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<i64>().unwrap();

    let v: Vec<i64> = (1..=n).collect();

    let mut n2 = 0;
    let mut n5 = 0;

    for i in v {
        let mut x = i;
        while x % 5 == 0 {
            n5 += 1;
            x /= 5;
        }

        let mut x = i;
        while x % 2 == 0 {
            n2 += 1;
            x /= 2;
        }
    }

    println!("{}", std::cmp::min(n2, n5));
}