fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let s = buffer.split_ascii_whitespace().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut queue = (1..=s[0]).collect::<std::collections::VecDeque<i32>>();
    let k = s[1];

    let mut res: Vec<i32> = vec![];

    while !queue.is_empty() {
        for _ in 0..k {
            let x = queue.pop_front();
            queue.push_back(x.unwrap());
        }
        res.push(queue.pop_back().unwrap());
    }

    use std::io::Write;

    let stdout = std::io::stdout();
    let mut stdout: std::io::BufWriter<std::io::StdoutLock<'_>> = std::io::BufWriter::new(stdout.lock());

    write!(stdout, "<").unwrap();
    write!(stdout, "{}", res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")).unwrap();
    write!(stdout, ">").unwrap();
}


