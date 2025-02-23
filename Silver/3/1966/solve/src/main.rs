fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let t = buffer.trim().parse::<i64>().unwrap();

    for _ in 0..t {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let mut case = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let mut queue = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<std::collections::VecDeque<i64>>();

        let mut cnt = 1;

        while queue.len() != 0 {
            let mut print = true;

            for i in 0..queue.len() {
                if i != 0 && queue[i] > queue[0] {
                    let t = queue.pop_front().unwrap();
                    queue.push_back(t);
                    if case[1] != 0 { case[1] -= 1; } else { case[1] = queue.len() as i64 - 1; }
                    print = false;
                    break;
                }
            }

            if print {
                if case[1] == 0 {
                    println!("{cnt}");
                    break;
                } else {
                    queue.pop_front();
                    case[1] -= 1;
                    cnt += 1;
                }
            }
        }
    }
}
