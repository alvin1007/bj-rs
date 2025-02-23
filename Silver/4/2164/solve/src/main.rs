fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i32>().unwrap();

    let mut stack = (1..=n).collect::<std::collections::VecDeque<i32>>();

    while stack.len() != 1 {
        stack.pop_front();
        let t = stack.pop_front().unwrap();
        stack.push_back(t);
    }

    print!("{}", stack[0]);
}
