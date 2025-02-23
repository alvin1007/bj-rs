fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i64>().unwrap();

    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    let mut heap = BinaryHeap::new();

    use std::io::Write;

    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<i64>().unwrap();

        if n != 0 {
            heap.push(Reverse(n));

        } else {
            if heap.is_empty() {
                writeln!(stdout, "0").unwrap();
            } else {
                let Reverse(val) = heap.pop().unwrap();
                writeln!(stdout, "{}",  val).unwrap();
                
            }
        }
    }
}
