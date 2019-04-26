//! **Problem 4**: Largest palindrome product
//!
//! A palindromic number reads the same both ways. The largest palindrome made
//! from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.
//!
//! [Problem 4 on projecteuler.net](https://projecteuler.net/problem=4)
//!

use std::iter::repeat;

fn is_palindrome_base_10(n: usize) -> bool {
    let s = n.to_string();
    let sb = s.as_bytes();

    let mut a = 0;
    let mut b = sb.len() - 1;

    loop {
        if sb[a] != sb[b] {
            return false;
        }

        if b == 0 {
            return true;
        }

        a += 1;
        b -= 1;
    }
}

/// Calculate solution to Problem 4
pub fn solution() -> String {
    (100..1000).rev().flat_map(|a| repeat(a).zip((100..1000).rev()))
    .map(|(a, b)| a * b)
    .fold(0, |max, p| {
        if p > max && is_palindrome_base_10(p) {
            p
        } else {
            max
        }
    })
    .to_string()
}
