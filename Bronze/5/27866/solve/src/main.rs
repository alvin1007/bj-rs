fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    
    let mut bufn = String::new();
    std::io::stdin().read_line(&mut bufn).unwrap();

    let chars: Vec<char> = buffer.chars().collect();
    print!("{}", chars[bufn.trim().parse::<usize>().unwrap()-1]);
}
