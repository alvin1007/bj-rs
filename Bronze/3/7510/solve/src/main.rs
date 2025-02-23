fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    
    let n = buffer.trim().parse::<i32>().unwrap();

    for i in 1..=n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let l = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        println!("Scenario #{i}:");

        let a = l[0].pow(2);
        let b = l[1].pow(2);
        let c = l[2].pow(2);

        if a + b == c || a + c == b || b + c == a {
            println!("yes\n")
        } else {
            println!("no\n")
        }
    }
}
