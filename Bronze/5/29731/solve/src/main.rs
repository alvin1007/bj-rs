fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i32>().unwrap();

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim() {
            "Never gonna give you up" => { continue; },
            "Never gonna let you down" => { continue; },
            "Never gonna run around and desert you" => { continue; },
            "Never gonna make you cry" => { continue; }, 
            "Never gonna say goodbye" => { continue; },
            "Never gonna tell a lie and hurt you" => { continue; },
            "Never gonna stop" => { continue; },
            _ => { print!("Yes"); return; }
        }
    }

    print!("No")
}

