fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i32>().unwrap();

    use std::io::Write;

    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());

    let mut stack: Vec<i32> = vec![];

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let op = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

        match op[0] {
            "push" => { stack.push(op[1].parse::<i32>().unwrap()); },
            "pop" => {
                match stack.pop() {
                    Some(i) => { writeln!(stdout, "{i}").unwrap() },
                    None => { writeln!(stdout, "-1").unwrap() },
                }
            },
            "size" => {
                writeln!(stdout, "{}", stack.len()).unwrap()
            },
            "empty" => {
                if stack.is_empty() {
                    writeln!(stdout, "1").unwrap()
                } else {
                    writeln!(stdout, "0").unwrap()
                }
            },
            "top" => {
                match stack.last() {
                    Some(i) => { writeln!(stdout, "{}", *i).unwrap() },
                    None => { writeln!(stdout, "-1").unwrap() },
                }
            },
            _ => {},
        }
    }
}
