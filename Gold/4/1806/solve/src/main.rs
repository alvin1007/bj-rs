fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let cfg = buf.split_ascii_whitespace().flat_map(|x|x.parse::<usize>()).collect::<Vec<_>>();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: Vec<usize> = buf.split_ascii_whitespace().flat_map(|x|x.parse()).collect();

    let mut l = 0;
    let mut r = 0;
    let mut partial_sum = n[0];
    let mut len = cfg[0] + 1;

    while l <= r && r < cfg[0]  {
        if partial_sum < cfg[1] {
            r += 1;
            if r == cfg[0] { break; }
            partial_sum += n[r];
        } else {
            len = len.min(r - l + 1);
            partial_sum -= n[l];
            l += 1;
        }
    }

    print!("{}", if len != cfg[0] + 1 { len } else { 0 });
}
