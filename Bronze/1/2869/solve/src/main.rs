fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

    let a = s[0].parse::<i64>().unwrap();
    let b = s[1].parse::<i64>().unwrap();
    let v = s[2].parse::<i64>().unwrap();

    println!("{}", if (v - a) % (a - b) == 0 { 
        (v - a) / (a - b) + 1 
    } else { 
        (v - a) / (a - b) + 2 
    });
}
