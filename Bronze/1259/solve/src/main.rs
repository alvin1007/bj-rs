fn main() {
    loop {
       let mut buffer = String::new();
       std::io::stdin().read_line(&mut buffer).unwrap();
       let s = buffer.trim().to_string();
       if buffer.trim().parse::<i32>().unwrap() == 0 { break; }
    
       let n = s.len();
    
       let mut p = true;
    
       for i in 0..n {
          if s.as_bytes()[i] != s.as_bytes()[n - i - 1] {
             p = false;
          }
       }
    
       if p { println!("yes"); } else { println!("no"); }
    }
}