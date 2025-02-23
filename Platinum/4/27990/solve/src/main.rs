/// 분수 표현
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    fn new(n: i64, d: i64) -> Self {
        let mut fraction = Self { numerator: n, denominator: d };
        fraction.simplify(); // 생성 시 약분
        fraction
    }

    /// 약분 수행
    fn simplify(&mut self) {
        let gcd = Self::gcd(self.numerator.abs(), self.denominator.abs());
        if gcd == 0 { return; }
        self.numerator /= gcd;
        self.denominator /= gcd;

        // 분모를 양수로 유지
        if self.denominator < 0 {
            self.numerator = -self.numerator;
            self.denominator = -self.denominator;
        }
    }

    /// 최대공약수 계산 (유클리드 호제법)
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    /// +
    fn add(&mut self, other: &Self) {
        self.numerator = self.numerator*other.denominator + other.numerator * self.denominator; 
        self.denominator = self.denominator*other.denominator;
        self.simplify();
    }

    /// >
    fn greater(&mut self, other: &Self) -> bool {
        if self.denominator < 0 {
            self.numerator = -self.numerator;
            self.denominator = -self.denominator;
        }
        self.numerator*other.denominator > self.denominator*other.numerator
    }

    /// <=
    fn less_equal(&mut self, other: &Self) -> bool {
        if self.denominator < 0 {
            self.numerator = -self.numerator;
            self.denominator = -self.denominator;
        }
        self.numerator*other.denominator <= self.denominator*other.numerator
    }
}

fn main() {
    let config = input();

    let mut balls: Vec<((i64, i64), (i64, i64))> = vec![];

    for _ in 0..config[1] {
        let ball = input();
        balls.push(((ball[0]+config[0], ball[1]+config[0]), (ball[2], ball[3])));
    }

    {
        let mut cnt = 0;
        let mut i = 0;
        
        while i < config[1] {
            let mut j = i + 1;

            while j < config[1] {

                let ax = balls[i as usize].0.0;
                let bx = balls[j as usize].0.0;
                let cx = balls[i as usize].1.0;
                let dx = balls[j as usize].1.0;

                let ay = balls[i as usize].0.1;
                let by = balls[j as usize].0.1;
                let cy = balls[i as usize].1.1;
                let dy = balls[j as usize].1.1;

                cnt += solve((ax+bx, cx+dx), (ax-bx, cx-dx), (ay+by, cy+dy), (ay-by, cy-dy), config[0], config[2]).len();

                j += 1;
            }

            i += 1;
        }
        println!("{cnt}");
    }
}

fn input() -> Vec<i64> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

use std::collections::HashSet;
///
/// a + bt = 0 (mod 4N)
/// bt = -a + 4N
/// bt + a = 4N
/// 
/// bt - 4Nk = a 
/// 
fn solve(a: (i64, i64), b: (i64, i64), c: (i64, i64), d: (i64, i64), n: i64, limit: i64) -> Vec<Fraction> {
    let mut result: Vec<Fraction> = vec![];

    let xt1 = a.0 % (4*n) == 0;
    let xt2 = a.1 == 0;
    let xt3 = b.0 % (4*n) == 0;
    let xt4 = b.1 == 0;

    let yt1 = c.0 % (4*n) == 0;
    let yt2 = c.1 == 0;
    let yt3 = d.0 % (4*n) == 0;
    let yt4 = d.1 == 0;

    if (xt1 && xt2) || (xt3 && xt4) {
        let mut yresult: Vec<Fraction> = vec![];

        if yt2 {
            yresult = euclidean(d.0, d.1, n, limit);
        } else if yt4 {
            yresult = euclidean(c.0, c.1, n, limit);
        } else {
            yresult = euclidean(c.0, c.1, n, limit);
            yresult.append(&mut euclidean(d.0, d.1, n, limit));
        }

        // 중복 제거
        let unique_yresult: HashSet<_> = yresult.into_iter().collect();
        return unique_yresult.into_iter().collect();
    }

    if (yt1 && yt2) || (yt3 && yt4) {
        let mut xresult: Vec<Fraction> = vec![];

        if xt2 {
            xresult = euclidean(b.0, b.1, n, limit);
        } else if xt4 {
            xresult = euclidean(a.0, a.1, n, limit);
        } else {
            xresult = euclidean(a.0, a.1, n, limit);
            xresult.append(&mut euclidean(b.0, b.1, n, limit));
        }

        // 중복 제거
        let unique_xresult: HashSet<_> = xresult.into_iter().collect();
        return unique_xresult.into_iter().collect();
    }

    let mut xresult: Vec<Fraction> = vec![];

    if xt2 {
        xresult = euclidean(b.0, b.1, n, limit);
    } else if xt4 {
        xresult = euclidean(a.0, a.1, n, limit);
    } else {
        xresult = euclidean(a.0, a.1, n, limit);
        xresult.append(&mut euclidean(b.0, b.1, n, limit));
    }

    let mut yresult: Vec<Fraction> = vec![];

    if yt2 {
        yresult = euclidean(d.0, d.1, n, limit);
    } else if yt4 {
        yresult = euclidean(c.0, c.1, n, limit);
    } else {
        yresult = euclidean(c.0, c.1, n, limit);
        yresult.append(&mut euclidean(d.0, d.1, n, limit));
    }

    // 교집합 구하기: 두 벡터를 HashSet에 넣고 교집합을 구함
    let xset: HashSet<_> = xresult.into_iter().collect();
    let yset: HashSet<_> = yresult.into_iter().collect();
    let intersection: HashSet<_> = xset.intersection(&yset).cloned().collect();

    result.extend(intersection);
    return result;
}

fn euclidean(a: i64, b:i64, n: i64, limit: i64) -> Vec<Fraction> {
    let mut result: Vec<Fraction> = vec![];

    let zero = Fraction::new(0, 1);
    let limit = Fraction::new(limit, 1);
    let n4 = Fraction::new(4*n, b.abs());
    
    let mut t = Fraction::new(-a, b);
    while !t.greater(&zero) { t.add(&n4); }
    while t.less_equal(&limit) {
        result.push(t.clone());
        t.add(&n4);
    }

    result
}

