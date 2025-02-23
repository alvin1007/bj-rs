fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i32>().unwrap();

    let mut points = vec![];

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let s = buffer.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();
        points.push((s[0], s[1]));
    }

    let mut cnt = 0;

    for i in 0..points.len()-2 {
        let x = points[i];
        for j in i+1..points.len()-1 {
            let y = points[j];
            for k in j+1..points.len() {
                let z = points[k];

                let ab = (x.0 - y.0).pow(2) + (x.1 - y.1).pow(2);
                let bc = (y.0 - z.0).pow(2) + (y.1 - z.1).pow(2);
                let ca = (z.0 - x.0).pow(2) + (z.1 - x.1).pow(2);

                if (ab == bc + ca) || (bc == ab + ca) || (ca == ab + bc) {
                    cnt += 1;
                }
            }
        }
    }

    print!("{cnt}");
}
