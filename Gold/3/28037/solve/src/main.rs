fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i32>().unwrap();

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let change = buffer.split_ascii_whitespace().flat_map(|x|x.parse::<i8>()).collect::<Vec<i8>>();

        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let mut pos = buffer.split_ascii_whitespace().flat_map(|x|x.parse::<f64>()).collect::<Vec<f64>>();

        match (change[0], change[1]) {
            (1, 2) => {
                let x = pos[0];
                let y = pos[1];

                pos[0] = (x.powi(2) + y.powi(2)).sqrt();

                pos[1] = y.atan2(x);
                if pos[1] < 0. {
                    pos[1] += std::f64::consts::TAU;
                }
                if pos[1] >= std::f64::consts::TAU {
                    pos[1] -= std::f64::consts::TAU;
                }
            },
            (1, 3) => {
                let x = pos[0];
                let y = pos[1];
                let z = pos[2];

                pos[0] = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
                pos[1] = (z/pos[0]).acos();
                if pos[1].is_nan() {
                    pos[1] = 0.;
                }
                pos[2] = y.atan2(x);
                if pos[2] < 0. {
                    pos[2] += std::f64::consts::TAU;
                }
                if pos[2] >= std::f64::consts::TAU {
                    pos[2] -= std::f64::consts::TAU;
                }
            },
            (2, 1) => {
                let r = pos[0];
                let o = pos[1];

                pos[0] = r*o.cos();
                pos[1] = r*o.sin();
            },
            (2, 3) => {
                let r = pos[0];
                let o = pos[1];

                pos[0] = r*o.cos();
                pos[1] = r*o.sin();

                let x = pos[0];
                let y = pos[1];
                let z = pos[2];

                pos[0] = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
                pos[1] = (z/pos[0]).acos();
                if pos[1].is_nan() {
                    pos[1] = 0.;
                }
                pos[2] = y.atan2(x);
                if pos[2] < 0. {
                    pos[2] += std::f64::consts::TAU;
                }
                if pos[2] >= std::f64::consts::TAU {
                    pos[2] -= std::f64::consts::TAU;
                }
            },
            (3, 1) => {
                let r = pos[0];
                let t = pos[1];
                let o = pos[2];

                pos[0] = r*t.sin()*o.cos();
                pos[1] = r*t.sin()*o.sin();
                pos[2] = r*t.cos();
            },
            (3, 2) => {
                let r = pos[0];
                let t = pos[1];
                let o = pos[2];

                pos[0] = r*t.sin()*o.cos();
                pos[1] = r*t.sin()*o.sin();
                pos[2] = r*t.cos();

                let x = pos[0];
                let y = pos[1];

                pos[0] = (x.powi(2) + y.powi(2)).sqrt();
                pos[1] = y.atan2(x);
                if pos[1] < 0. {
                    pos[1] += std::f64::consts::TAU;
                }
                if pos[1] >= std::f64::consts::TAU {
                    pos[1] -= std::f64::consts::TAU;
                }
            },
            _ => {},
        }
        println!("{} {} {}", pos[0], pos[1], pos[2])
    }
}