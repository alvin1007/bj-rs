fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let d1 = buffer.trim().parse::<i32>().unwrap();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let d2 = buffer.trim().to_string();

    println!("{}", d1 * (d2.as_bytes()[2] - 48) as i32);
    println!("{}", d1 * (d2.as_bytes()[1] - 48) as i32);
    println!("{}", d1 * (d2.as_bytes()[0] - 48) as i32);
    println!("{}", d1 * d2.parse::<i32>().unwrap());
}
