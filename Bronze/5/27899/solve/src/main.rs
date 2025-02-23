fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    print!("{}", match buffer.trim() {
        "NLCS" => "North London Collegiate School",
        "BHA" => "Branksome Hall Asia",
        "KIS" => "Korea International School",
        "SJA" => "St. Johnsbury Academy",
        _ => "",
    })
}
