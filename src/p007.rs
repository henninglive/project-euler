//! **Problem 7**: 10001st prime
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11,
//! and 13, we can see that the 6th prime is 13.
//!
//! What is the 10 001st prime number?
//!
//! [Problem 7 on projecteuler.net](https://projecteuler.net/problem=7)
//!

struct Primes(usize);

fn is_prime(num: usize) -> bool {
    let max = (num as f64).sqrt().ceil() as usize;
    for n in 2..=max {
        if num % n == 0 {
            return false;
        }
    }
    true
}

impl Primes {
    fn new() -> Primes {
        Primes(1)
    }
}

impl Iterator for Primes {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        loop {
            self.0 += 1;
            if self.0 <= 3 || is_prime(self.0) {
                return Some(self.0);
            }
        }
    }
}

/// Calculate solution to Problem 7
pub fn solution() -> String {
    Primes::new().nth(10000).unwrap().to_string()
}

#[test]
fn test_solution() {
    assert_eq!("104743", solution());
}
