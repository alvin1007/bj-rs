fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let l = buf.trim().parse::<usize>().unwrap();

    let mut lo = vec![true; l];

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i64>().unwrap();

    let mut max = (0, 0);
    let mut peo = (0, 0);

    for i in 1..=n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

        if max.0 < s[1] - s[0] { max.0 = s[1] - s[0]; max.1 = i; }

        let mut j = s[0] - 1;

        let mut cnt = 0;

        while j < s[1] {
            if lo[j as usize] {
                cnt += 1;
                lo[j as usize] = false;
            }
            j += 1;
        }

        if peo.0 < cnt {
            peo.0 = cnt;
            peo.1 = i;
       }
    }

    println!("{}", max.1);
    println!("{}", peo.1);
}