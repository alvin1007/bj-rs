fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let t = buffer.trim().parse::<i64>().unwrap();

    for i in 1..=t {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let n = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        if n[0] + n[1] <= n[2] || n[1] + n[2] <= n[0] || n[0] + n[2] <= n[1] {
            println!("Case #{}: invalid!", i);
            continue;
        }

        if n[0] == n[1] && n[1] == n[2] {
            println!("Case #{}: equilateral", i);
            continue;
        }

        if n[0] == n[1] || n[1] == n[2] || n[0] == n[2] {
            println!("Case #{}: isosceles", i);
            continue;
        }

        println!("Case #{}: scalene", i)
    }
}
