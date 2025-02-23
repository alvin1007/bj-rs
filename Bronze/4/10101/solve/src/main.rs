fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let a = buffer.trim().parse::<i64>().unwrap();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let b = buffer.trim().parse::<i64>().unwrap();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let c = buffer.trim().parse::<i64>().unwrap();

    if a + b + c != 180 {
        print!("Error");
        return;
    }

    if a != b && b != c && a != c {
        print!("Scalene");
        return;
    }

    if a == 60 && b == 60 && c == 60 {
        print!("Equilateral");
        return;
    }

    print!("Isosceles");
}
