fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();
    let a = vec!["1", "2", "3", "4", "5", "6", "7", "8"];
    let b = vec!["8", "7", "6", "5", "4", "3", "2", "1"];
    if s.eq(&a) {
        print!("ascending");
    } else if s.eq(&b) {
        print!("descending");
    } else {
        print!("mixed");
    }
}
