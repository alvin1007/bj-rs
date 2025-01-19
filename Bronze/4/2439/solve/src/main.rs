fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut i = 0;
    
    while i < n {
        let mut j = 0;

        while j < n-i-1 {
            print!(" ");
            j += 1;
        }

        j = 0;
        while j < i + 1 {
            print!("*");
            j += 1;
        }
        print!("\n");

        i += 1;
    }
}
