//! # Smallest multiple
//! ## Problem 5
//!
//! 2520 is the smallest number that can be divided by each of the numbers from
//! 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the
//! numbers from 1 to 20?
//!

use ::util::Factorize;

use std::collections::HashMap;

pub fn p5() -> String {
    let mut min = HashMap::new();
    for i in 1..20+1 {
        let factors = Factorize::new(i).fold(
            HashMap::<usize, usize, _>::new(),
            |mut map, f| {
                *map.entry(f).or_insert(0) += 1;
                map
            }
        );

        for (f, n) in factors {
            let mn = min.entry(f).or_insert(0);
            *mn = ::std::cmp::max(n, *mn);
        }
    }

    min.into_iter()
        .fold(1, |p, (f, n)| p * f.pow(n as u32))
        .to_string()
}