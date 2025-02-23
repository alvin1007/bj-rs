fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let str = buf.trim().to_string();
    let mut str = str.bytes();

    let mut stack = vec![];

    let mut ord = std::collections::HashMap::new();
    ord.insert(b'(', 0);
    ord.insert(b'+', 1);
    ord.insert(b'-', 1);
    ord.insert(b'*', 2);
    ord.insert(b'/', 2);

    let mut res = vec![];

    loop {
        let a = match str.next() {
            Some(b) => b,
            None => { break; }
        };

        match a {
            b'+' | b'-' | b'*' | b'/' => {
                while !stack.is_empty() && ord.get(stack.last().unwrap()) >= ord.get(&a) {
                    res.push(stack.pop().unwrap());
                }
                stack.push(a);
            },
            b'(' => {
                stack.push(a);
            },
            b')' => {
                while *stack.last().unwrap() != b'(' {
                    res.push(stack.pop().unwrap());
                }
                stack.pop();
            },
            _ => {
                res.push(a);
            },
        }
    }

    while !stack.is_empty() {
        res.push(stack.pop().unwrap());
    }

    print!("{}", String::from_utf8(res).unwrap());
}
