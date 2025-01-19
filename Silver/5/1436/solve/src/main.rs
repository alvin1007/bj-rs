fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut n = buffer.trim().parse::<i64>().unwrap();

    let mut res = 666;

    loop {
        if check(res) {
            n -= 1;
        }

        if n == 0 { break; }

        res += 1;
    }

    println!("{}", res);
}

fn check(s: i64) -> bool {
    let mut i = 0;
    for c in s.to_string().as_bytes() {
        if *c == b'6' {
            i += 1;
        } else {
            i = 0;
        }

        if i == 3 {
            return true;
        }
    }
    return false;
}