//! **Problem 20**: Factorial digit sum
//!
//! n! means n × (n − 1) × ... × 3 × 2 × 1
//! For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
//! and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//!
//! Find the sum of the digits in the number 100!
//! 
//! [Problem 20 on projecteuler.net](https://projecteuler.net/problem=20)
//!

use num::bigint::ToBigUint;

/// Calculate solution to Problem 20
pub fn solution() -> String {
    (2..=100)
        .fold(1.to_biguint().unwrap(), |p, i| p * i.to_biguint().unwrap())
        .to_string()
        .chars()
        .map(|c| c. to_digit(10).unwrap())
        .sum::<u32>()
        .to_string()
}
