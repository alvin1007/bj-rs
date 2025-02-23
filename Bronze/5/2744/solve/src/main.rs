fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let s = buffer.trim().as_bytes().iter().map(|x| if *x < 91 { *x + 32 } else { *x - 32 }).collect::<Vec<u8>>();
    
    for t in s {
        print!("{}", t as char)
    }
}
