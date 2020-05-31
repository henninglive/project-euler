//! **Problem 25**: 1000-digit Fibonacci number
//!
//! The Fibonacci sequence is defined by the recurrence relation:
//!
//! Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
//!
//! Hence the first 12 terms will be:
//! 1. 1
//! 2. 1
//! 3. 2
//! 4. 3
//! 5. 5
//! 6. 8
//! 7. 13
//! 8. 21
//! 9. 34
//! 10. 55
//! 11. 89
//! 12. 144
//!
//! The 12th term, F12, is the first term to contain three digits.
//! What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
//!
//! [Problem 25 on projecteuler.net](https://projecteuler.net/problem=25)
//!

use num::BigUint;

use crate::util::Fibonacci;

/// Calculate solution to Problem 25
pub fn solution() -> String {
    let max = BigUint::parse_bytes("9".repeat(999).as_bytes(), 10)
        .unwrap();

    (Fibonacci::<BigUint>::one()
        .enumerate()
        .find(|(_, n)| n > &max)
        .unwrap()
        .0 + 1) // enumerate is zero indexed
        .to_string()
}

#[test]
fn test_solution() {
    assert_eq!("4782", solution());
}
