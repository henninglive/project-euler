//! **Problem 16**: Power digit sum
//!
//! `2^15 = 32768` and the sum of its digits is `3 + 2 + 7 + 6 + 8 = 26`.
//! What is the sum of the digits of the number `2^1000`?
//!
//! [Problem 16 on projecteuler.net](https://projecteuler.net/problem=16)
//!

use num::{BigUint, one};

/// Calculate solution to Problem 16
pub fn solution() -> String {
    let mut n = one::<BigUint>();
    n <<= 1000;
    n.to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>()
        .to_string()
}