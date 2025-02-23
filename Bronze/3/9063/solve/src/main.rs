fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    
    let n = buffer.trim().parse::<i64>().unwrap();

    let mut xmin = 10000;
    let mut xmax = -10000;
    let mut ymin = 10000;
    let mut ymax = -10000;

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let l = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        xmin = xmin.min(l[0]);
        xmax = xmax.max(l[0]);
        ymin = ymin.min(l[1]);
        ymax = ymax.max(l[1]);
    }

    print!("{}", (xmax - xmin) * (ymax - ymin));
}
