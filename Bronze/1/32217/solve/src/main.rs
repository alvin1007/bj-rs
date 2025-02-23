fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i32>()).collect::<Vec<i32>>();

    println!("{}", 180 * (s.len() as i32 - 1) - s.iter().sum::<i32>() * 2)
}
