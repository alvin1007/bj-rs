fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    match buffer.trim().parse::<i8>().unwrap() {
        1 => { println!("Leading the Way to the Future") },
        0 => { println!("YONSEI") },
        _ => {},
    }
}
