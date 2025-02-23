fn main() {
    let pi = 3.141592;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let d1 = buffer.trim().parse::<f64>().unwrap();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let d2 = buffer.trim().parse::<f64>().unwrap();

    print!("{}", d1*2. + 2. * pi * d2);
}
