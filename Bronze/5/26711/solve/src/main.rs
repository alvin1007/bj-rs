fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let s = buffer.trim();

    let a = BigInt::new(s);

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let s = buffer.trim();

    let b = BigInt::new(s);

    let mut t = a.add(&b);
    t.digits.reverse();

    for s in t.digits {
        print!("{s}")
    }
}

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

    fn add_abs(a: &Self, b: &Self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut carry = 0;
        let max_len = a.digits.len().max(b.digits.len());

        for i in 0..max_len {
            let digit_a = if i < a.digits.len() { a.digits[i] } else { 0 };
            let digit_b = if i < b.digits.len() { b.digits[i] } else { 0 };
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

        result
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

    fn add(&self, other: &Self) -> Self {
        match (self.sign, other.sign) {
            (true, true) => BigInt {
                digits: Self::add_abs(self, other),
                sign: true,
            },
            (false, false) => BigInt {
                digits: Self::add_abs(self, other),
                sign: false,
            },
            (true, false) => self.subtract(&BigInt {
                digits: other.digits.clone(),
                sign: true,
            }),
            (false, true) => other.subtract(&BigInt {
                digits: self.digits.clone(),
                sign: true,
            }),
        }
    }

    fn subtract(&self, other: &Self) -> Self {
        match (self.sign, other.sign) {
            // Case 1: Both numbers are positive
            (true, true) => match self.compare_abs(other) {
                Ordering::Greater => BigInt {
                    digits: Self::subtract_abs(self, other).digits,
                    sign: true,
                },
                Ordering::Less => BigInt {
                    digits: Self::subtract_abs(other, self).digits,
                    sign: false,
                },
                Ordering::Equal => BigInt::new("0"), // 0 if equal
            },
    
            // Case 2: Both numbers are negative
            (false, false) => match self.compare_abs(other) {
                Ordering::Greater => BigInt {
                    digits: Self::subtract_abs(self, other).digits,
                    sign: false,
                },
                Ordering::Less => BigInt {
                    digits: Self::subtract_abs(other, self).digits,
                    sign: true,
                },
                Ordering::Equal => BigInt::new("0"), // 0 if equal
            },
    
            // Case 3: Self is positive, other is negative
            (true, false) => self.add(&BigInt {
                digits: other.digits.clone(),
                sign: true,
            }),
    
            // Case 4: Self is negative, other is positive
            (false, true) => {
                let result = self.add(&BigInt {
                    digits: other.digits.clone(),
                    sign: false,
                });
                result
            }
        }
    }
}
