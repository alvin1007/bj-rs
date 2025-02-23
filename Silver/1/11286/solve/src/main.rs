fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i64>().unwrap();

    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap_positive = BinaryHeap::new();
    let mut heap_negative = BinaryHeap::new();

    use std::io::Write;

    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<i64>().unwrap();

        if n != 0 {
            if n < 0 {
                heap_negative.push(n);
            } else {
                heap_positive.push(Reverse(n));
            }

        } else {
            if heap_positive.is_empty() && heap_negative.is_empty() {
                writeln!(stdout, "0").unwrap();
            } else {
                if heap_positive.is_empty() {
                    writeln!(stdout, "{}", heap_negative.pop().unwrap()).unwrap();
                    continue;
                }

                if heap_negative.is_empty() {
                    let Reverse(val) = heap_positive.pop().unwrap();
                    writeln!(stdout, "{}", val).unwrap();
                    continue;
                }

                let Reverse(val) = heap_positive.peek().unwrap();
                if *val >= -heap_negative.peek().unwrap() {
                    writeln!(stdout, "{}", heap_negative.pop().unwrap()).unwrap();
                } else {
                    writeln!(stdout, "{}", *val).unwrap();
                    heap_positive.pop();
                }
            }
        }
    }
}
