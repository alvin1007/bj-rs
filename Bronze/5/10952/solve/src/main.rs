use std::io::stdin;

fn main() {
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
    
        let numbers = buffer.split_ascii_whitespace().collect::<Vec<&str>>();
    
        let a = numbers[0].parse::<usize>().unwrap();
        let b = numbers[1].parse::<usize>().unwrap();
    
        if a == 0 { return; }
    
        println!("{}", a + b);
    }
}
