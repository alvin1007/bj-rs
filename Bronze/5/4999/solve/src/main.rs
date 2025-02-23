fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    
    let s = buffer.trim().len() as i32;

    buffer.clear();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    
    let t = buffer.trim().len() as i32;

    if t - s > 0 { println!("no") } else { println!("go") }
}
