fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let numbers: Vec<i64> = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut n = 2_i64.pow(numbers[0] as u32);
    n /= 2;

    // (c, r)
    let mut r = numbers[1];
    let mut c = numbers[2];

    let mut res: Vec<i64> = vec![];

    while n >= 1 {

        let i = if n < c + 1 {
            if n < r + 1 { r -= n; c -= n; 3 } else { c -= n; 1} 
        } else {
            if n < r + 1 { r -= n; 2 } else { 0}
        };

        res.push(i);

        n /= 2;
    }

    let mut cnt = 0;
    let mut ptr = 0;
    for i in res.iter().rev() {
        ptr += i * 4_i64.pow(cnt);
        cnt += 1;
    }

    println!("{}", ptr);
}
