fn main() {
    let mut max = 0;
    let mut pos = 0;

    let mut i = 0;

    while i < 9 {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let n = buffer.trim().parse::<i32>().unwrap();
        if max < n {
            max = n;
            pos = i;
        }
        i += 1;
    }

    println!("{}", max);
    println!("{}", pos+1);
}
