fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let k = buffer.trim().parse::<i32>().unwrap();

    let mut stack: Vec<i64> = vec![];

    for _ in 0..k {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let n = buffer.trim().parse::<i64>().unwrap();
        if n != 0 {
            stack.push(n);
        } else {
            stack.pop();
        }
    }

    print!("{}", stack.iter().sum::<i64>())
}
