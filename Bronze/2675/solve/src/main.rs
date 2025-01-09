fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let t = buffer.trim().parse::<i64>().unwrap();

    let mut i = 0;

    while i < t {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).unwrap();
        let strs: Vec<&str> = str.split_ascii_whitespace().collect();
        
        let mut j = 0;

        let s = strs[1].to_string();

        while j < s.len() {

            let mut k = 0;

            while k < strs[0].parse::<i64>().unwrap() {
                print!("{}", s.chars().nth(j).unwrap());
                k += 1;
            }

            j += 1;
        }
        print!("\n");

        strs[0].parse::<i64>().unwrap();
        i += 1;
    }
}
