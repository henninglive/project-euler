//! **Problem 43**: Lexicographic permutations
//!
//! The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of
//! the digits 0 to 9 in some order, but it also has a rather interesting sub-string
//! divisibility property.
//!
//! Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:
//! * d2d3d4=406 is divisible by 2
//! * d3d4d5=063 is divisible by 3
//! * d4d5d6=635 is divisible by 5
//! * d5d6d7=357 is divisible by 7
//! * d6d7d8=572 is divisible by 11
//! * d7d8d9=728 is divisible by 13
//! * d8d9d10=289 is divisible by 17
//!
//! Find the sum of all 0 to 9 pandigital numbers with this property.
//!
//! [Problem 43 on projecteuler.net](https://projecteuler.net/problem=43)
//!

use crate::util::Permutations;
use num::{BigUint, Zero};

#[inline]
fn divisible(p: &[u8], a: usize, b: usize, c: usize, divisor: u32) -> bool {
    (p[a] as u32 * 100 + p[b] as u32 * 10 + p[c] as u32) % divisor == 0
}

/// Calculate solution to Problem 43
pub fn solution() -> String {
    let mut digits = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut perms = Permutations::new(&mut digits);
    let mut sum = BigUint::zero();

    while let Some(p) = perms.next() {
        if !divisible(p, 1, 2, 3, 2) {
            continue;
        }
        if !divisible(p, 2, 3, 4, 3) {
            continue;
        }
        if !divisible(p, 3, 4, 5, 5) {
            continue;
        }
        if !divisible(p, 4, 5, 6, 7) {
            continue;
        }
        if !divisible(p, 5, 6, 7, 11) {
            continue;
        }
        if !divisible(p, 6, 7, 8, 13) {
            continue;
        }
        if !divisible(p, 7, 8, 9, 17) {
            continue;
        }
        sum += BigUint::from_radix_be(&p[..], 10).unwrap();
    }

    sum.to_string()
}

#[test]
fn test_solution() {
    assert_eq!("16695334890", solution());
}
