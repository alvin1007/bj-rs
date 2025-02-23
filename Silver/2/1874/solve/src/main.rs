fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let n = next().parse::<i64>().unwrap();

    let mut stk: Vec<i64> = vec![];

    let mut res: Vec<u8> = vec![];
    let mut tmp = 1;

    for _ in 0..n {
        let num = next().parse::<i64>().unwrap();

        while tmp <= num {
            res.push(b'+');
            stk.push(tmp);
            tmp += 1;
        }

        if *stk.last().unwrap() == num {
            stk.pop();
            res.push(b'-');
        } else {
            writeln!(stdout, "NO").unwrap();
            print!("{stdout}");
            return;
        }
    }
    for s in res {
        writeln!(stdout, "{}", s as char).unwrap();
    }
    print!("{stdout}")
}
