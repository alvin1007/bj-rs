use std::cmp::min;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut board: Vec<String> = vec![];

    for _ in 0..n[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        board.push(buffer.trim().to_string());
    }

    let mut re = -1;

    for i in 0..=n[0]-8 {
        for j in 0..=n[1]-8 {
            let t = find(board.clone(), (j, i));
            if re == -1 || re > t { re = t; } 
        }
    }

    print!("{re}")
}

fn find(board: Vec<String>, start: (i32, i32)) -> i32 {
    let change = |x| match x {
        b'W' => b'B',
        b'B' => b'W',
        _ => b'E',
    };

    let mut case1 = 0;
    let mut f = b'W';

    for i in start.1..start.1+8 {
        let str = board[i as usize].as_bytes();
        for j in start.0..start.0+8 {
            if str[j as usize] != f {
                case1 += 1;
            }
            f = change(f);
        }
        f = change(f);
    }

    let mut case2 = 0;
    let mut f = b'B';

    for i in start.1..start.1+8 {
        let str = board[i as usize].as_bytes();
        for j in start.0..start.0+8 {
            if str[j as usize] != f {
                case2 += 1;
            }
            f = change(f);
        }
        f = change(f);
    }

    return min(case1, case2);
}