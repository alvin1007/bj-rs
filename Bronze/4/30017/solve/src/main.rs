fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let s = buffer.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    if s[0] <= s[1] + 1 {
        print!("{}", s[0] * 2 - 1)
    } else {
        print!("{}", s[1] * 2 + 1);
    }
}
