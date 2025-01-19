fn main() {
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

        let a = s[0].parse::<i64>().unwrap().pow(2);
        let b = s[1].parse::<i64>().unwrap().pow(2);
        let c = s[2].parse::<i64>().unwrap().pow(2);

        if a == 0 {
            return;
        }

        if a + b == c || a + c == b || b + c == a {
            println!("right");
        } else {
            println!("wrong");
        }
    }
}
