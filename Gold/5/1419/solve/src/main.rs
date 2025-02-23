fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let l = buffer.trim().parse::<i128>().unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let r = buffer.trim().parse::<i128>().unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let k = buffer.trim().parse::<i128>().unwrap();

    if k % 2 != 0 {
        let t = (k + 1) >> 1;
        let ll = (l + k - 1) / k;
        let rr = r / k;
        if t > rr { println!("0") }
        else { println!("{}", rr - ll.max(t) + 1) }
    } else {
        let t = k + 1;
        let ll = (l + k / 2 - 1) / (k / 2);
        let rr = r / (k / 2);
        let m = if k == 4 { if ll <= 6 && 6 <= rr { 1 } else { 0 } } else { 0 };
        if t > rr { println!("0") }
        else { println!("{}", rr - ll.max(t) + 1 - m) }
    }
}
