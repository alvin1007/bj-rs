fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let d = next().parse::<f64>().unwrap();
    let h = next().parse::<f64>().unwrap();
    let w = next().parse::<f64>().unwrap();

    let k = ((d*d)/((h*h)+(w*w))).sqrt();

    write!(stdout, "{} {}", (h*k) as i32, (w*k) as i32).unwrap();

    print!("{stdout}");
}
