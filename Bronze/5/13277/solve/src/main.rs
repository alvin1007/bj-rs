#[derive(Clone, Copy)]
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
}

#[derive(Debug, Clone)]
struct BigInt {
    digits: Vec<i64>, // Least significant digit first
    sign: bool,      // true for positive, false for negative
}

impl BigInt {
    fn new(value: &str) -> Self {
        let sign = !value.starts_with('-');
        let digits = value
            .trim_start_matches('-')
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        BigInt { digits, sign }
    }

    fn fft(p: &mut Vec<Complex>, inv: bool) {
        let s = p.len();

        let mut j = 0;
        for i in 1..s {
            let mut bit = s / 2;

            while j >= bit { j -= bit; bit >>= 1; }
            j += bit;

            if i < j { p.swap(i, j); }
        }

        let mut k = 1;
        while k < s {
            let angle = if inv { std::f64::consts::PI / k as f64 } else { -std::f64::consts::PI / k as f64 };
            let dir = Complex::new(angle.cos(), angle.sin());

            let mut i = 0;
            while i < s {
                let mut unit = Complex::new(1., 0.);

                for j in 0..k {
                    let a = p[i + j];
                    let b = p[i + j + k].mul(&unit);

                    p[i + j] = a.add(&b);
                    p[i + j + k] = a.sub(&b);

                    unit = unit.mul(&dir);
                }

                i += k << 1;
            }

            k <<= 1;
        }

        if inv {
            let cs = Complex::new(s as f64, 0.);
            for i in 0..s {
                p[i] = p[i].div(&cs);
            }
        }
    }

    // use fft
    fn multiply(&self, other: &Self) -> Self {
        let mut a = vec![];
        let mut b = vec![];

        for i in 0..self.digits.len() { a.push(Complex::new(self.digits[i] as f64, 0.)); }
        for i in 0..other.digits.len() { b.push(Complex::new(other.digits[i] as f64, 0.)); }

        let mut n = 2;

        while n < a.len() + b.len() { n <<= 1; }

        a.resize(n, Complex::new(0., 0.));
        b.resize(n, Complex::new(0., 0.));

        Self::fft(&mut a, false);
        Self::fft(&mut b, false);

        let mut w= vec![Complex::new(0., 0.); n];

        for i in 0..n {
            w[i] = a[i].mul(&b[i]);
        }

        Self::fft(&mut w, true);

        let mut res = vec![0; w.len()];

        for i in 0..res.len() {
            res[i] = w[i].x.round() as i64;
        }

        for i in 0..res.len() {
            if res[i] < 10 { continue; }
            if i < res.len() - 1 { res[i + 1] += res[i] / 10; }
            else { res.push(res[i] / 10); }

            res[i] %= 10;
        }

        let mut i = (res.len() - 1) as i64;
        while !res.is_empty() && res[i as usize] == 0 { res.pop(); i -= 1; }
        if res.is_empty() { res.push(0); }

        BigInt {
            digits: res,
            sign: !(self.sign ^ other.sign),
        }
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let s = buf.split_ascii_whitespace().collect::<Vec<&str>>();

    let a = BigInt::new(s[0]);
    let b = BigInt::new(s[1]);

    let mut add = a.multiply(&b);
    add.digits.reverse();

    use std::fmt::Write;
    let mut stdout = String::new();

    if add.sign {
        for d in add.digits {
            write!(stdout, "{}", d).unwrap();
        }
    } else {
        for d in add.digits {
            write!(stdout, "{}", d).unwrap();
        }
    }

    print!("{stdout}")
}