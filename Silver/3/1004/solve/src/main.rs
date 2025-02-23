/// 2가지 케이스
/// 1. 원이 출발점을 감싸고 도착점을 감싸지 않는 경우 
/// 2. 원이 도착점을 감싸고 출발점을 감싸지 않는 경우
fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let t = buffer.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        // (x, y)
        let start_end = input();

        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let s = buffer.trim().parse::<i32>().unwrap();

        // (x, y, r)
        let mut planet: Vec<Vec<i32>> = vec![];

        for _ in 0..s {
            planet.push(input());
        }

        let mut cnt = 0;

        for p in planet {
            let case1 = p[2] as f32 > distant((start_end[0], start_end[1]), (p[0], p[1]));
            let case2 = p[2] as f32 > distant((start_end[2], start_end[3]), (p[0], p[1]));

            if case1 && case2 {
                continue;
            }

            if case1 {
                cnt += 1;
                continue;
            }

            if case2 {
                cnt += 1;
                continue;
            }
        }

        println!("{cnt}")
    }
}

fn input() -> Vec<i32> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn distant(a: (i32, i32), b: (i32, i32)) -> f32 {
    (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f32).sqrt()
}