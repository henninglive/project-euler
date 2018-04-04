//! **Problem 9**: Special Pythagorean triplet
//!
//! A Pythagorean triplet is a set of three natural numbers, `a < b < c`, for
//! which, `a2 + b2 = c2`
//!
//! For example, `32 + 42 = 9 + 16 = 25 = 52`.
//! There exists exactly one Pythagorean triplet for which `a + b + c = 1000`.
//! Find the product `abc`.
//!
//! [Problem 9 on projecteuler.net](https://projecteuler.net/problem=9)
//!

/// Calculate solution to Problem 9
pub fn solution() -> String {
    for a in 1..1000 {
        for b in a..1000 {
            let c = 1000 - (a + b);
            if a * a + b * b == c * c {
                return (a*b*c).to_string()
            }
        }
    }
    unreachable!();
}