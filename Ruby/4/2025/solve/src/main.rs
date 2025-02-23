use std::io;

const MOVES: [(i32, i32); 8] = [
    (2, 1), (1, 2), (-1, 2), (-2, 1),
    (-2, -1), (-1, -2), (1, -2), (2, -1),
];

fn is_valid_move(x: i32, y: i32, board: &Vec<Vec<i32>>, n: i32) -> bool {
    x >= 0 && y >= 0 && x < n && y < n && board[y as usize][x as usize] == 0
}

fn get_degree(x: i32, y: i32, board: &Vec<Vec<i32>>, n: i32) -> i32 {
    let mut c = 0;
    for pos in MOVES {
        if is_valid_move(x + pos.0, y + pos.1, board, n) {
            c += 1;
        }
    }
    return c;
}

fn warnsdorff_knight_tour(n: i32, start_x: i32, start_y: i32) -> Option<Vec<(i32, i32)>> {
    let mut board = vec![vec![0; n as usize]; n as usize];
    let mut x = start_x;
    let mut y = start_y;
    let mut path = Vec::new();

    board[y as usize][x as usize] = 1;
    path.push((x, y));

    for i in 2..n*n+1 {
        let mut possible: Vec<(i32, i32)> = vec![];
        for pos in MOVES {
            if is_valid_move(x+pos.0, y+pos.1, &board, n) {
                possible.push((x+pos.0, y+pos.1));
            }
        }

        if possible.len() == 0 { return None; }

        let mut min = 10;
        let mut move_point = (-1, -1);

        for p in possible {
            let degree = get_degree(p.0, p.1, &board, n);
            if degree <= min {
                if degree == min {
                    if (p.0 - n / 2).pow(2) + (p.1 - n / 2).pow(2) < (move_point.0 - n / 2).pow(2) + (move_point.1 - n / 2).pow(2) {
                        continue;
                    }
                }
                min = degree;
                move_point = p;
            }
        }

        x = move_point.0;
        y = move_point.1;
        board[y as usize][x as usize] = i;

        path.push(move_point);
    }

    return Some(path);
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let coords: Vec<i32> = buf.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (start_x, start_y) = (coords[0], coords[1]);

    use std::fmt::Write;
    let mut stdout = String::new();

    match warnsdorff_knight_tour(n, start_x - 1, start_y - 1) {
        Some(path) => {
            for (x, y) in path {
                writeln!(stdout, "{} {}", x + 1, y + 1).unwrap();
            }
        }
        None => writeln!(stdout, "-1 -1").unwrap(),
    }

    print!("{stdout}")
}