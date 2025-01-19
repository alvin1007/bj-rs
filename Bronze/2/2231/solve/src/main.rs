fn main() {
    
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let s = buffer.trim().to_string();

    let mut r = buffer.trim().parse::<i64>().unwrap();

    let n = s.len();
    let mut conv: Vec<i64> = vec![];

    for i in (1..=n).rev() {
        conv.push(10_i64.pow(i as u32)/10 + 1);
    }

    let mut store: Vec<i64> = vec![];

    let mut df = true;
    for i in (1..=n).rev() {
        let mut st = 0;
        let b = 10_i64.pow(i as u32)/10 + 1;
        let a = conv[n-i+1..n].iter().sum::<i64>();

        for _ in 1..=9 {
            if 9*a >= r {
                if i == 2 && r%2 != 0 && r >= 11 {} else { break; }
            }
            st += 1;
            r -= b;
        }

        if i == 1 && r%2 != 0 {
            print!("0");
            df = false;
            break;
        } 
        store.push(st);
    }

    if df {
        println!("{}", store.into_iter().map(|i| i.to_string()).collect::<String>().parse::<i64>().unwrap());
    }


}