fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let _ = buffer.trim().parse::<i32>().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let s = buffer.split_terminator("").skip(1).collect::<Vec<&str>>();
    
    let mut t = 0;
    for num in s {
        t += match num.parse::<i32>() {
            Ok(i) => i,
            Err(_) => 0,
        };
    }

    print!("{}", t);
}
