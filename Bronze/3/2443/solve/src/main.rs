fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n= buf.trim().parse::<usize>().unwrap();

    for i in 0..n {
        print!("{}", " ".repeat(i));
        println!("{}", "*".repeat(2*n - 2*i - 1));
    }
}
