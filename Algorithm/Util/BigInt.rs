use std::cmp::Ordering;
use std::fmt;

pub fn iterable_to_str<I, D>(iterable: I) -> String
where
    I: IntoIterator<Item = D>,
    D: fmt::Display,
{
    let mut iterator = iterable.into_iter();

    let head = match iterator.next() {
        None => return String::from(""),
        Some(x) => format!("{}", x),
    };
    let body = iterator.fold(head, |a, v| format!("{}{}", a, v));
    format!("{}", body)
}

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
                Ordering::Equal => BigInt::new("0"),
            },
    
            (false, false) => match self.compare_abs(other) {
                Ordering::Greater => BigInt {
                    digits: Self::subtract_abs(self, other).digits,
                    sign: false,
                },
                Ordering::Less => BigInt {
                    digits: Self::subtract_abs(other, self).digits,
                    sign: true,
                },
                Ordering::Equal => BigInt::new("0"),
            },
    
            (true, false) => self.add(&BigInt {
                digits: other.digits.clone(),
                sign: true,
            }),

            (false, true) => {
                let result = self.add(&BigInt {
                    digits: other.digits.clone(),
                    sign: false,
                });
                result
            }
        }
    }

    fn multiply(&self, other: &Self) -> Self {
        let mut result = vec![0; self.digits.len() + other.digits.len()];

        for (i, &a) in self.digits.iter().enumerate() {
            let mut carry = 0;
            for (j, &b) in other.digits.iter().enumerate() {
                let product = a as u16 * b as u16 + result[i + j] as u16 + carry;
                result[i + j] = (product % 10) as u8;
                carry = (product / 10) as u16;
            }

            if carry > 0 {
                result[i + other.digits.len()] += carry as u8;
            }
        }

        while result.last() == Some(&0) && result.len() > 1 {
            result.pop();
        }

        BigInt {
            digits: result,
            sign: self.sign == other.sign,
        }
    }
}