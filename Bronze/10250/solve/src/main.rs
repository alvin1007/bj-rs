fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let t = buffer.trim().parse::<i32>().unwrap();

    let mut i = 0;

    while i < t {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

        let h = s[0].parse::<i64>().unwrap();
        // let w = s[1].parse::<i64>().unwrap();
        let n = s[2].parse::<i64>().unwrap();

        if n % h == 0 {
            if n / h < 10 {
                println!("{}0{}", h, n/h)
            } else {
                println!("{}{}", h , n/h)
            }
        } else {
            if n / h + 1 < 10 {
                println!("{}0{}", n % h, n/h+1)
            } else {
                println!("{}{}", n % h, n/h+1)
            }
        }

        i += 1;
    }
}
