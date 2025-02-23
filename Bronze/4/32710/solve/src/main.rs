fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n= buf.trim().parse::<i64>().unwrap();
    
    if n < 10 {
        println!("1");
        return;
    }

    if n % 2 == 0 && n / 2 < 10 {
        println!("1");
        return;
    }
    if n % 3 == 0 && n / 3 < 10 {
        println!("1");
        return;
    }
    if n % 4 == 0 && n / 4 < 10 {
        println!("1");
        return;
    }
    if n % 5 == 0 && n / 5 < 10 {
        println!("1");
        return;
    }
    if n % 6 == 0 && n / 6 < 10 {
        println!("1");
        return;
    }
    if n % 7 == 0 && n / 7 < 10 {
        println!("1");
        return;
    }
    if n % 8 == 0 && n / 8 < 10 {
        println!("1");
        return;
    }
    if n % 9 == 0 && n / 9 < 10 {
        println!("1");
        return;
    }

    println!("0")
}
