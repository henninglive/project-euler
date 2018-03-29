//! # Power digit sum
//! ## Problem 16
//!
//! 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//! What is the sum of the digits of the number 2^1000?
//!

use ::num_bigint::BigUint;
use ::num_traits::one;

pub fn p016() -> String {
    let mut n = one::<BigUint>();
    n <<= 1000;
    n.to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>()
        .to_string()
}