use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let t = buffer.trim().parse::<usize>().unwrap();

    let mut i = 0;
    while i < t {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let nums: Vec<&str> = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

        println!("{}", nums[0].parse::<i32>().unwrap() + nums[1].parse::<i32>().unwrap());
        i += 1;
    }
}
