fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut nums = buf.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    nums.sort();

    let mut sum = 0;
    for i in 0..nums.len() {
        sum += nums[0..=i].iter().sum::<i64>();
    }

    println!("{sum}");
}