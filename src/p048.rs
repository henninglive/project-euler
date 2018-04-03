//! # Self powers
//! ## Problem 48
//!
//! The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
//! Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
//!

use ::num_bigint::BigUint;
use ::num_traits::{FromPrimitive, zero};

pub fn p048() -> String {
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
