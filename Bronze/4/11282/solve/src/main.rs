fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let t = buf.trim().parse::<u16>().unwrap();
    print!("{}", String::from_utf16(&[0xAC00 + t - 1]).unwrap())
}
