fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n1 = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n2 = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n3 = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    print!("{} ", 
        if n1[0] == n2[0] {
            n3[0]
        } else if n2[0] == n3[0] {
            n1[0]
        } else {
            n2[0]
        }
    );

    println!("{}", 
        if n1[1] == n2[1] {
            n3[1]
        } else if n2[1] == n3[1] {
            n1[1]
        } else {
            n2[1]
        }
    );
}
