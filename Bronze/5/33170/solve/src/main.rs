fn main() {
    use std::io::Read;
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    print!("{}", if buffer.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>().into_iter().sum::<i32>() <= 21 { 1 } else { 0 });
}
