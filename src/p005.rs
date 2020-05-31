//! **Problem 5**: Smallest multiple
//!
//! 2520 is the smallest number that can be divided by each of the numbers from
//! 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the
//! numbers from 1 to 20?
//!
//! [Problem 5 on projecteuler.net](https://projecteuler.net/problem=5)
//!

use std::collections::HashMap;

use crate::util::Factorize;

/// Calculate solution to Problem 5
pub fn solution() -> String {
    (1..=20)
        .fold(HashMap::new(), |mut min, i| {
            let factors = Factorize::new(i).fold(
                HashMap::<usize, usize, _>::new(),
                |mut map, f| {
                    *map.entry(f).or_insert(0) += 1;
                    map
                },
            );

            for (f, n) in factors {
                let mn = min.entry(f).or_insert(0);
                *mn = ::std::cmp::max(n, *mn);
            }
            min
        })
        .into_iter()
        .fold(1, |p, (f, n)| p * f.pow(n as u32))
        .to_string()
}

#[test]
fn test_solution() {
    assert_eq!("232792560", solution());
}
