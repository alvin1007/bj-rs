fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<usize>().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    
    let nums: Vec<&str> = buffer.split_ascii_whitespace().collect();

    let mut min: i64 = 1000000;
    let mut max: i64 = -1000000;

    let mut i = 0;

    while i < n {
        let num = nums[i].parse::<i64>().unwrap();

        if num < min {
            min = num;
        }

        if num > max {
            max = num;
        }

        i += 1;
    }

    print!("{} {}", min, max);
}
