//! **Problem 52**: Permuted multiples
//!
//! It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits, but in a different order.
//! Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.
//!
//! [Problem 52 on projecteuler.net](https://projecteuler.net/problem=52)
//!

fn digits(mut num: usize) -> [u8; 10] {
    let mut d = [0u8; 10];
    while num > 0 {
        d[num % 10] += 1;
        num /= 10;
    }
    d
}

/// Calculate solution to Problem 52
pub fn solution() -> String {
    (1..).find(|i| {
        let d = digits(*i);
        (2..=6).all(|j| d == digits(i * j))
    })
    .unwrap()
    .to_string()
}
