//! **Problem 48**: Self powers
//!
//! The series, `1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317`.
//! Find the last ten digits of the series, `1^1 + 2^2 + 3^3 + ... + 1000^1000`.
//!
//! [Problem 48 on projecteuler.net](https://projecteuler.net/problem=48)
//!

use num::{BigUint, FromPrimitive, zero};

/// Calculate solution to Problem 48
pub fn solution() -> String {
    let mod_digits = BigUint::from_u64(10_000_000_000).unwrap();

    let sum = (1..(1000 + 1))
        .map(|i| {
            let n = BigUint::from_u64(i).unwrap();
            n.modpow(&n, &mod_digits)
        })
        .fold(zero::<BigUint>(), |mut sum, i| {
            sum += i;
            sum
        });

    (sum % mod_digits).to_str_radix(10)
}
