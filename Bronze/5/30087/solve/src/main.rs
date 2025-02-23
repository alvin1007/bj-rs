fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<i32>().unwrap();

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim() {
            "Algorithm" => { println!("204") },
            "DataAnalysis" => { println!("207") },
            "ArtificialIntelligence" => { println!("302") },
            "CyberSecurity" => { println!("B101") },
            "Network" => { println!("303") },
            "Startup" => { println!("501") },
            "TestStrategy" => { println!("105") },
            _ => {},
        }
    }
}
