fn main() {
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
    
        let n = buffer.split_ascii_whitespace().flat_map(|x|x.parse::<f64>()).collect::<Vec<f64>>();

        if n[0] == 0. && n[1] == 0. {
            println!("AXIS");
            break;
        }

        if n[0] == 0. || n[1] == 0. {
            println!("AXIS");
            continue;
        }

        match (n[0] > 0., n[1] > 0.) {
            (true, true) => { println!("Q1"); continue; },
            (true, false) => { println!("Q4"); continue; },
            (false, true) => { println!("Q2"); continue; },
            (false, false) => { println!("Q3"); continue; },
        }
    }
}
