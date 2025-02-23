fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let c: usize = buf.trim().parse().unwrap();

    for _ in 0..c {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let values: Vec<i64> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (l, v1, v2, t, s) = (values[0], values[1], values[2], values[3], values[4]);

        let mut k = 0;
        let mut nk = 1;
        let mut vk = v2;
        let mut possible = true;
        while t * nk < vk - v1 {
            let vfk = l as f64 / (s * (k + 1)) as f64;
            let mut nfk = 0;
            if vk as f64 > vfk + t as f64 {
                nfk = ((vk as f64 - vfk - t as f64) / t as f64).ceil() as i64;
            }
            nk = 2*(nk - nfk);
            vk = vk - nfk * t;
            if nk <= 0 { possible = false; break; }
            k += 1;
        }

        if possible { println!("{}", k) }
        else { println!("impossible") }
    }
}