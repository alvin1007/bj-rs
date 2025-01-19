use std::iter::successors;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let a = buffer.trim().parse::<i64>().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let b = buffer.trim().parse::<i64>().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let c = buffer.trim().parse::<i64>().unwrap();

    println!("{}", a + b - c);
    println!("{}", a*10_i64.pow(successors(Some(b), |&b| (b >= 10).then(|| b / 10)).count() as u32) + b - c);
}
