//! # Power digit sum
//! ## Problem 16
//!
//! 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//! What is the sum of the digits of the number 2^1000?
//!

#[derive(Debug)]
struct Base10(Vec<u8>);

impl Base10 {
    fn one() -> Base10 {
        Base10(vec![1])
    }

    fn pow2(&mut self) {
        let mut carry = 0;
        for i in self.0.iter_mut() {
            let n = (*i as usize) * 2 + carry;
            *i = (n % 10) as u8;
            carry = n / 10;
        }

        while carry > 0 {
            self.0.push((carry % 10) as u8);
            carry = carry / 10;
        }
    }

    fn sum_digits(&self) -> usize {
        self.0.iter().map(|i| *i as usize).sum()
    }
}

pub fn p016() -> String {
    let mut n = Base10::one();
    for _ in 0..1000 {
        n.pow2();
    }

    n.sum_digits().to_string()
}