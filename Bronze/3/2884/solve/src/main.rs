fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

    let mut h: i32 = s[0].parse::<i32>().unwrap();
    let mut m: i32 = s[1].parse::<i32>().unwrap();

    m -= 45;

    if m < 0 { 
        h -= 1;
        m += 60;
    }
    if h < 0 { 
        h += 24;
    }
    
    println!("{} {}", h, m);
}
