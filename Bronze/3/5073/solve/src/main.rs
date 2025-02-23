fn main() {
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let n = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        if n[0] == 0 && n[1] == 0 && n[2] == 0 {
            break;
        }

        if n[0] + n[1] <= n[2] || n[1] + n[2] <= n[0] || n[0] + n[2] <= n[1] {
            println!("Invalid");
            continue;
        }

        if n[0] == n[1] && n[1] == n[2] {
            println!("Equilateral");
            continue;
        }

        if n[0] == n[1] || n[1] == n[2] || n[0] == n[2] {
            println!("Isosceles");
            continue;
        }

        println!("Scalene")
    }
}
