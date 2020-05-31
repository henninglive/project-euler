//! **Problem 1**: Multiples of 3 and 5
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5,
//! we get 3, 5, 6 and 9. The sum of these multiples is 23.
//! Find the sum of all the multiples of 3 or 5 below 1000.
//!
//! [Problem 1 on projecteuler.net](https://projecteuler.net/problem=1)
//!

/// Calculate solution to Problem 1
pub fn solution() -> String {
    (1..1000)
        .filter(|i| i % 3 == 0 || i % 5 == 0)
        .sum::<usize>()
        .to_string()
}

#[test]
fn test_solution() {
    assert_eq!("233168", solution());
}
