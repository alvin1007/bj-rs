
// N, M
fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    
    let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>().iter().flat_map(|x| x.parse::<i64>()).collect::<Vec<i64>>();
    
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    
    let cards = buffer.split_ascii_whitespace().collect::<Vec<&str>>().iter().flat_map(|x| x.parse::<i64>()).collect::<Vec<i64>>();
    
    let mut res = 0;
    for i in 0..cards.len()-2 {
        for j in i+1..cards.len()-1 {
            for k in j+1..cards.len() {
                let a = cards[i] + cards[j] + cards[k];
                if s[1] >= a && res <= a {
                    res = a;
                }
            }
        }
    }

    println!("{}", res);
}   
