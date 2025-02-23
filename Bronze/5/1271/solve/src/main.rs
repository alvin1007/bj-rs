use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct BigInt {
    digits: Vec<u8>, // Least significant digit first
    sign: bool,      // true for positive, false for negative
}

impl BigInt {
    fn new(value: &str) -> Self {
        let sign = !value.starts_with('-');
        let digits = value
            .trim_start_matches('-')
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        BigInt { digits, sign }
    }

    fn compare_abs(&self, other: &Self) -> Ordering {
        match self.digits.len().cmp(&other.digits.len()) {
            Ordering::Equal => {
                for (a, b) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
                    match a.cmp(b) {
                        Ordering::Equal => continue,
                        non_eq => return non_eq,
                    }
                }
                Ordering::Equal
            }
            non_eq => non_eq,
        }
    }

    fn subtract_abs(a: &Self, b: &Self) -> Self {
        let mut result = Vec::new();
        let mut borrow = 0;
        for i in 0..a.digits.len() {
            let digit_a = a.digits[i];
            let digit_b = if i < b.digits.len() { b.digits[i] } else { 0 };
            let mut diff = digit_a as i16 - digit_b as i16 - borrow;

            if diff < 0 {
                diff += 10;
                borrow = 1;
            } else {
                borrow = 0;
            }

            result.push(diff as u8);
        }

        while result.last() == Some(&0) {
            result.pop();
        }

        if result.is_empty() {
            result.push(0);
        }

        BigInt {
            digits: result,
            sign: true,
        }
    }

    fn subtract(&self, other: &Self) -> Self {
        match (self.sign, other.sign) {
            (true, true) => match self.compare_abs(other) {
                Ordering::Greater => BigInt {
                    digits: Self::subtract_abs(self, other).digits,
                    sign: true,
                },
                Ordering::Less => BigInt {
                    digits: Self::subtract_abs(other, self).digits,
                    sign: false,
                },
                Ordering::Equal => BigInt::new("0"),
            },
            (false, false) => BigInt {
                digits: Self::subtract_abs(other, self).digits,
                sign: true,
            },
            (true, false) => {
                let mut sum = self.clone();
                sum.add(&BigInt { digits: other.digits.clone(), sign: true })
            }
            (false, true) => {
                let mut sum = other.clone();
                sum.add(&BigInt { digits: self.digits.clone(), sign: true })
            }
        }
    }

    fn add(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        let mut carry = 0;
        let max_len = self.digits.len().max(other.digits.len());

        for i in 0..max_len {
            let digit_a = if i < self.digits.len() { self.digits[i] } else { 0 };
            let digit_b = if i < other.digits.len() { other.digits[i] } else { 0 };
            let mut sum = digit_a as u16 + digit_b as u16 + carry;

            if sum >= 10 {
                sum -= 10;
                carry = 1;
            } else {
                carry = 0;
            }

            result.push(sum as u8);
        }

        if carry > 0 {
            result.push(carry as u8);
        }

        BigInt {
            digits: result,
            sign: self.sign,
        }
    }

    fn divide_with_remainder(&self, other: &Self) -> (Self, Self) {
        if other.digits.is_empty() || (other.digits.len() == 1 && other.digits[0] == 0) {
            panic!("Division by zero");
        }

        let mut remainder = BigInt::new("0");
        let mut quotient = Vec::new();

        for &digit in self.digits.iter().rev() {
            remainder.digits.insert(0, digit);
            while remainder.digits.last() == Some(&0) && remainder.digits.len() > 1 {
                remainder.digits.pop();
            }

            let mut count = 0;
            while remainder.compare_abs(other) != Ordering::Less {
                remainder = BigInt::subtract_abs(&remainder, other);
                count += 1;
            }

            quotient.push(count as u8);
        }

        quotient.reverse();
        while quotient.last() == Some(&0) && quotient.len() > 1 {
            quotient.pop();
        }

        if remainder.digits.is_empty() {
            remainder.digits.push(0);
        }

        remainder.sign = self.sign;
        (
            BigInt {
                digits: quotient,
                sign: self.sign == other.sign,
            },
            remainder,
        )
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let s = buffer.split_ascii_whitespace().collect::<Vec<&str>>();

    let a = BigInt::new(s[0]);
    let b = BigInt::new(s[1]);

    let (mut result_div, mut result_rem) = a.divide_with_remainder(&b);
    result_div.digits.reverse();
    result_rem.digits.reverse();
    
    for s in result_div.digits {
        print!("{s}")
    }

    print!("\n");

    for s in result_rem.digits {
        print!("{s}")
    }
}
