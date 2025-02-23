fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n= buf.trim().parse::<usize>().unwrap();

    for _ in 0..n {
        println!("{}", "@".repeat(n*5))
    }

    for _ in 0..n*3 {
        println!("{}", "@".repeat(n))
    } 

    for _ in 0..n {
        println!("{}", "@".repeat(n*5))
    }
}
