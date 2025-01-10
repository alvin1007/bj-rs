fn main() {
    let mut mem: Vec<i32> = vec![];

    for _ in 0..10 {
        let s = input_number() % 42;
        if !mem.contains(&s) {
            mem.push(s);
        }
    }

    print!("{}", mem.len());
}

fn input_number() -> i32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().parse::<i32>().unwrap()
}