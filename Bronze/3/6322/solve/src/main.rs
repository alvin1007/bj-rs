fn main() {
    let mut cnt = 1;
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let n = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        if n[0] == 0 && n[1] == 0 && n[2] == 0 {
            break;
        }

        println!("Triangle #{}", cnt);

        if n[0] == -1 {
            if n[2] <= n[1] {
                println!("Impossible.\n")
            } else {
                println!("a = {:.3}\n", ((n[2].pow(2) - n[1].pow(2)) as f64).sqrt())
            }
        } else if n[1] == -1 {
            if n[2] <= n[0] {
                println!("Impossible.\n")
            } else {
                println!("b = {:.3}\n", ((n[2].pow(2) - n[0].pow(2)) as f64).sqrt())
            }
        } else {
            println!("c = {:.3}\n", ((n[0].pow(2) + n[1].pow(2)) as f64).sqrt())
        }

        cnt += 1;
    }
}
