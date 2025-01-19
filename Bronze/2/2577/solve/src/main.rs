fn main() {
    let a = input_number();
    let b = input_number();
    let c = input_number();

    let str = format!("{}", a*b*c);

    let num = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for n in num {
        let mut cnt = 0;
        for s in str.chars() {
            if n == s { cnt += 1; }
        } 
        println!("{}", cnt);
    }
}

fn input_number() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<i64>().unwrap()
}