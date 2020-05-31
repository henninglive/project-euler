//! **Problem 56**: Powerful digit sum
//!
//! A googol (10100) is a massive number: one followed by one-hundred zeros; 100100 is almost unimaginably large: one followed by 
//! two-hundred zeros. Despite their size, the sum of the digits in each number is only 1.
//!
//! Considering natural numbers of the form, ab, where a, b < 100, what is the maximum digital sum?
//!
//! [Problem 56 on projecteuler.net](https://projecteuler.net/problem=56)
//!

use num::bigint::ToBigUint;
use num::pow;
use std::iter::repeat;

/// Calculate solution to Problem 56
pub fn solution() -> String {
    (2..100usize)
        .flat_map(|a| (2..100usize).zip(repeat(a)))
        .map(|(a, b)| pow(a.to_biguint().unwrap(), b))
        .map(|i| i.to_str_radix(10).chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>())
        .max()
        .unwrap()
        .to_string()
}

#[test]
fn test_solution() {
    assert_eq!("972", solution());
}
