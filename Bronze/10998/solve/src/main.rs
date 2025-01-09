fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let nums: Vec<&str> = buffer.split_ascii_whitespace().collect();
    print!("{}", nums[0].parse::<usize>().unwrap() * nums[1].parse::<usize>().unwrap());
}
