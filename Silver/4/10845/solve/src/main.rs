fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i32>().unwrap();

    use std::io::Write;

    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());

    let mut queue =  std::collections::VecDeque::<i32>::new();

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let op = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

        match op[0] {
            "push" => { queue.push_back(op[1].parse::<i32>().unwrap()); },
            "pop" => {
                match queue.pop_front() {
                    Some(i) => { writeln!(stdout, "{i}").unwrap() },
                    None => { writeln!(stdout, "-1").unwrap() },
                }
            },
            "size" => {
                writeln!(stdout, "{}", queue.len()).unwrap()
            },
            "empty" => {
                if queue.is_empty() {
                    writeln!(stdout, "1").unwrap()
                } else {
                    writeln!(stdout, "0").unwrap()
                }
            },
            "front" => {
                match queue.front() {
                    Some(i) => { writeln!(stdout, "{}", *i).unwrap() },
                    None => { writeln!(stdout, "-1").unwrap() },
                }
            },
            "back" => {
                match queue.back() {
                    Some(i) => { writeln!(stdout, "{}", *i).unwrap() },
                    None => { writeln!(stdout, "-1").unwrap() },
                }
            },
            _ => {},
        }
    }
}

