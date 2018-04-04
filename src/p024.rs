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

pub struct Permutations<'a, T: 'a>(&'a mut [T], bool);

impl<'a, T: Ord> Permutations<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Permutations<'a, T> {
        assert!(slice.len() > 0);
        slice.sort();
        Permutations(slice, true)
    }

    pub fn next<'b>(&'b mut self) -> Option<&'b mut [T]> {
        if self.1 {
            self.1 = false;
            return Some(self.0);
        }

        // Find non-increasing suffix
        let mut i = self.0.len() - 1;
        while i > 0 && self.0[i - 1] >= self.0[i] {
            i -= 1;
        }

        if i == 0 {
            return None;
        }

        // Find successor to pivot
        let mut j = self.0.len() - 1;
        while self.0[j] <= self.0[i - 1] {
            j -= 1;
        }

        self.0.swap(i - 1, j);

        // Reverse suffix
        self.0[i..].reverse();
        Some(self.0)
    }
}

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

