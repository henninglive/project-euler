//! **Problem 10**: Summation of primes
//!
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//! Find the sum of all the primes below two million.
//!
//! [Problem 10 on projecteuler.net](https://projecteuler.net/problem=10)
//!

use bit_vec::BitVec;

const MAX: usize = 2_000_000;

/// Calculate solution to Problem 10
pub fn solution() -> String {
    let mut bv = BitVec::from_elem(MAX, true);
    bv.set(0, false);
    bv.set(1, false);

    for i in 2..(1 + (MAX as f64).sqrt() as usize) {
        if bv[i] {
            for j in i.. {
                if i * j >= MAX {
                    break;
                }
                bv.set(i * j, false)
            }
        }
    }

    bv.into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i) } else { None })
        .sum::<usize>()
        .to_string()
}

#[test]
fn test_solution() {
    assert_eq!("142913828922", solution());
}
