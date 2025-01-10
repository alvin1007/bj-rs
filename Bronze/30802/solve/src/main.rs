fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<i64>().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let c = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

    let t = s[0].parse::<i64>().unwrap();
    let p = s[1].parse::<i64>().unwrap();

    let mut res1 = 0;
    for x in c {
        let a = x.parse::<i64>().unwrap();

        res1 += if a%t != 0 { a/t + 1 } else { a/t };
    }

    println!("{}", res1);

    println!("{} {}", n/p, n%p);
}
