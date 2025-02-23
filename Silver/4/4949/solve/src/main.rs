fn main() {
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        if buffer.as_bytes()[0] == b'.' && buffer.trim().len() == 1 { break; }

        let str = buffer.trim();

        let mut stack: Vec<u8> = vec![];

        let mut fail = false;

        for s in str.as_bytes() {
            if *s == b'(' || *s == b'[' {
                stack.push(*s);
            } else if *s == b')' {
                if stack.is_empty() || *stack.last().unwrap() != b'(' {
                    fail = true;
                    break;
                } else {
                    stack.pop();
                }
            } else if *s == b']' {
                if stack.is_empty() || *stack.last().unwrap() != b'[' {
                    fail = true;
                    break;
                } else {
                    stack.pop();
                }
            }
        }

        if fail {
            println!("no");
            continue;
        }

        if stack.is_empty() {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
