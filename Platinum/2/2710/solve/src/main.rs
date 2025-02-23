struct Complex {
    x: f64,
    y: f64,
}

impl Complex {
    fn new(x: f64, y: f64) -> Self {
        Self {
            x: x,
            y: y,
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }

    fn div(&self, other: &Self) -> Self {
        let d = other.x.powi(2) + other.y.powi(2);
        Self {
            x: (self.x * other.x + self.y * other.y) / d,
            y: (self.y * other.x - self.x * other.y) / d,
        }
    }

    fn sqrt(&self) -> Self {
        let magnitude = (self.x * self.x + self.y * self.y).sqrt();
        let real_part = ((magnitude + self.x) / 2.0).sqrt();
        let imag_part = self.y.signum() * ((magnitude - self.x) / 2.0).sqrt();
        Self {
            x: real_part,
            y: imag_part,
        }
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i64>().unwrap();

    let three = Complex::new(3., 0.);

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let s = buf.split_ascii_whitespace().flat_map(|x|x.parse::<f64>()).collect::<Vec<f64>>();

        let a = Complex::new(s[0], s[1]);
        let b = Complex::new(s[2], s[3]);
        let c = Complex::new(s[4], s[5]);

        let c_a = a.add(&b).add(&c);
        let c_b = a.mul(&b).add(&b.mul(&c)).add(&c.mul(&a));

        let r = c_a.mul(&c_a).sub(&three.mul(&c_b)).sqrt();

        let mut f1 = c_a.add(&r).div(&three);
        let mut f2 = c_a.sub(&r).div(&three);

        if f1.x > f2.x { std::mem::swap(&mut f1, &mut f2); }
        if f1.x == f2.x && f1.y > f2.y { std::mem::swap(&mut f1, &mut f2); }

        let fx1 = f1.x;
        let fy1 = f1.y;
        let fx2 = f2.x;
        let fy2 = f2.y;

        let x = (a.x + b.x) / 2.;
        let y = (a.y + b.y) / 2.;

        let mut rl = 0.;
        rl += ((x - fx1).powi(2) + (y - fy1).powi(2)).sqrt();
        rl += ((x - fx2).powi(2) + (y - fy2).powi(2)).sqrt();

        println!("{:.2} {:.2} {:.2} {:.2} {:.2}", (fx1 * 100.).round() / 100., (fy1 * 100.).round() / 100., (fx2 * 100.).round() / 100., (fy2 * 100.).round() / 100., (rl * 100.).round() / 100.);
    }
}
