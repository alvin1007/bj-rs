fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let t = buffer.trim().parse::<i64>().unwrap();

    for _ in 0..t {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let case = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let p1 = (case[0], case[1]);
        let p2 = (case[3], case[4]);
        let r1 = case[2];
        let r2 = case[5];

        if p1 == p2 && r1 == r2 {
            println!("-1");
            continue;
        }

        let dis = (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2);

        if dis > (r1 - r2).pow(2) && dis < (r1 + r2).pow(2) {
            println!("2");
            continue;
        }

        if (r1 + r2).pow(2) == dis || (r1 - r2).pow(2) == dis {
            println!("1");
            continue;
        }

        println!("0");
    }
}
