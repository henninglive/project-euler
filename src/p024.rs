//! **Problem 24**: Lexicographic permutations
//!
//! A permutation is an ordered arrangement of objects.
//! For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4.
//! If all of the permutations are listed numerically or alphabetically,
//! we call it lexicographic order. The lexicographic permutations of 0, 1 and
//! 2 are: 012, 021, 102, 120, 201, 210
//!
//! What is the millionth lexicographic permutation of the digits
//! 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
//!
//! [Problem 24 on projecteuler.net](https://projecteuler.net/problem=24)
//!

use crate::util::Permutations;

/// Calculate solution to Problem 24
pub fn solution() -> String {
    let mut numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut perms = Permutations::new(&mut numbers);

    let mut n = 1;
    while let Some(p) = perms.next() {
        if n == 1_000_000 {
            return p.iter()
                .map(|i| i.to_string())
                .collect::<String>();
        }
        n += 1;
    }

    unreachable!();
}

#[test]
fn test_solution() {
    assert_eq!("2783915460", solution());
}
