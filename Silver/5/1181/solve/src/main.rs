fn main() {
    let n = input().parse::<i64>().unwrap();
    let mut strs: Vec<String> = vec![];
    for _ in 0..n {
        let inp = input();

        let mut a = 0;

        for s in &strs {
            if s.len() >= inp.len() {
                break;
            }
            a += 1;
        }

        if a >= strs.len() {
            strs.push(inp);
            continue;
        }

        let mut equal = false;

        for s in &strs[a..] {
            if s.len() > inp.len() {
                break;
            }

            if s.eq(&inp) { equal = true; break; }

            let mut i = 0;
            let mut re = false;

            for c in s.as_bytes() {
                if *c > inp.as_bytes()[i]  {
                    break;
                } else if *c < inp.as_bytes()[i] {
                    re = true;
                    break;
                }
                i += 1;
            }

            if re { a += 1; } else { break; }
        }

        if equal { continue; }

        if a >= strs.len() {
            strs.push(inp);
            continue;
        }

        strs.insert(a, inp);
    }

    for s in strs {
        println!("{}", s);
    }
}

fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().to_string();
}