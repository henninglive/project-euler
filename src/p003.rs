//! **Problem 3**: Largest prime factor
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//! What is the largest prime factor of the number 600851475143 ?
//!
//! [Problem 3 on projecteuler.net](https://projecteuler.net/problem=3)
//!

use crate::util::Factorize;

/// Calculate solution to Problem 1
pub fn solution() -> String {
    let mut factors = Factorize::new(600_851_475_143).collect::<Vec<_>>();
    factors.sort();
    factors.last()
        .map(|i| i.to_string())
        .unwrap()
}

#[test]
fn test_solution() {
    assert_eq!("6857", solution());
}
