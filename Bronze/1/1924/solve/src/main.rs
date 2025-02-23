fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n= buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
    
    let mut cnt = 0;

    match n[0] {
        1 => {},
        2 => { cnt += 31 },
        3 => { cnt += 31 + 28 },
        4 => { cnt += 31 + 28 + 31 },
        5 => { cnt += 31 + 28 + 31 + 30 },
        6 => { cnt += 31 + 28 + 31 + 30 + 31 },
        7 => { cnt += 31 + 28 + 31 + 30 + 31 + 30 },
        8 => { cnt += 31 + 28 + 31 + 30 + 31 + 30 + 31 },
        9 => { cnt += 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 },
        10 => { cnt += 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 },
        11 => { cnt += 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 },
        12 => { cnt += 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30 },
        _ => {},
    }
    cnt += n[1];

    match cnt % 7 {
        1 => { println!("MON") },
        2 => { println!("TUE") },
        3 => { println!("WED") },
        4 => { println!("THU") },
        5 => { println!("FRI") },
        6 => { println!("SAT") },
        0 => { println!("SUN") },
        _ => {}
    }
}
