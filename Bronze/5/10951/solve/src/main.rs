use std::io::stdin;

fn main() {
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        let numbers: Vec<&str> = buffer.split_ascii_whitespace().collect();

        if numbers.len() == 0 {
            return;
        }

        println!("{}", numbers[0].parse::<usize>().unwrap() + numbers[1].parse::<usize>().unwrap())
    }
}
