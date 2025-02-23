fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let cfg = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut lines: Vec<i64> = vec![];

    for _ in 0..cfg[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        lines.push(buffer.trim().parse::<i64>().unwrap());
    }

    let mut min = 0;
    let mut max = lines.iter().max().unwrap() + 1;

    while min < max {
        let mid = (max + min) / 2;

        let mut  cnt = 0;

        for i in 0..lines.len() {
            cnt += lines[i] / mid;
        }

        if cnt < cfg[1] {
            max = mid;
        } else {
            min = mid + 1;
        }
    }

    print!("{}", min - 1);
}
