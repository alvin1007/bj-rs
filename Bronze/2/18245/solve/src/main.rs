fn main() {
    let mut cnt = 2;
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let n= buf.split_ascii_whitespace().collect::<Vec<&str>>();
        
        if n.len() == 6 {
            if 
            n[0] == "Was" &&
            n[1] == "it" &&
            n[2] == "a" &&
            n[3] == "cat" &&
            n[4] == "I" &&
            n[5] == "saw?" { break; }
        }

        let str = n[0].to_string();
        let byte= str.as_bytes();

        for &s in byte.iter().step_by(cnt) {
            print!("{}", s as char)
        }
        print!("\n");
        cnt += 1;
    }
}
