fn main() {
    let a = input();
    let b = input();
    let c = input();

    let res = if a >= b {
        if b <= c {
            if a >= c { c } else { a }
        } else {
            b
        }
    } else {
        if b <= c {
            b
        } else {
            if a >= c { a } else { c }
        }
    };

    println!("{}", res);
}

fn input() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<i64>().unwrap()
}
