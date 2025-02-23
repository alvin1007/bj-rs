fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i32>().unwrap();

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let s = buffer.trim().to_string();

        let mut stack = 0;
        for c in s.as_bytes() {
            match *c {
                b'(' => { stack += 1; },
                b')' => { stack -= 1; },
                _ => {},
            }

            if stack < 0 { break; }
        }

        if stack == 0 {
            println!("YES")
        } else {
            println!("NO")
        }
    }
}
