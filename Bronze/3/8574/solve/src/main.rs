fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    
    let n = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut cnt = 0;

    let k = n[1].pow(2);

    for _ in 0..n[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let l = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        if (l[0] - n[2]).pow(2) + (l[1] - n[3]).pow(2) > k {
            cnt += 1;
        }  
    }

    print!("{cnt}");
}
