fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let t = buf.trim().parse::<i64>().unwrap();
    
    for _ in 0..t {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let s = buf.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        match s[1] {
            2 | 4 | 6 | 8 | 9 | 11 => {
                println!("{} {} {}", s[0], s[1] - 1, 31);
            },
            5 | 7 | 10 | 12 => {
                println!("{} {} {}", s[0], s[1] - 1, 30);
            },
            3 => {
                let k = (s[0] % 400 == 0) || (s[0] % 100 != 0 && s[0] % 4 == 0);
                if k {
                    println!("{} {} {}", s[0], s[1] - 1, 29);    
                } else {
                    println!("{} {} {}", s[0], s[1] - 1, 28);
                }
            },
            1 => {
                println!("{} {} {}", s[0] - 1, 12, 31);
            },
            _ => {},
        }
    }
}
