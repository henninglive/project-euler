//! **Problem 14**: Longest Collatz sequence
//!
//! The following iterative sequence is defined for the set of positive integers:
//! n → n/2 (n is even)
//! n → 3n + 1 (n is odd)
//!
//! Using the rule above and starting with 13, we generate the following sequence:
//! 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
//! 
//! It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
//! Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
//!
//! Which starting number, under one million, produces the longest chain?
//! NOTE: Once the chain starts the terms are allowed to go above one million.
//!
//! [Problem 14 on projecteuler.net](https://projecteuler.net/problem=14)
//!

struct Collatz(usize);

impl Iterator for Collatz {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.0 < 2 {
            return None;
        }

        let next = match self.0 % 2 {
            0 => self.0 / 2,
            _ => (3 * self.0) + 1,
        };
        self.0 = next;
        Some(next)
    }
}


/// Calculate solution to Problem 14
pub fn solution() -> String {
    (1..1_000_000)
        .map(|n| (Collatz(n).count(), n))
        .max_by_key(|e| e.0)
        .unwrap().1
        .to_string()
}
