fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let p1 = buffer.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let p2 = buffer.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let p3 = buffer.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    print!("{}", ccw((p1[0], p1[1]), (p2[0], p2[1]), (p3[0], p3[1])))
}

fn ccw(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> i32 {
    let mut res = p1.0*p2.1 + p2.0*p3.1 + p3.0*p1.1;
    res -= p2.0*p1.1 + p3.0*p2.1 + p1.0*p3.1;
    return if res > 0 { 1 } else { 0 } - if res < 0 { 1 } else { 0 };
}
