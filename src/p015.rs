//! # Lattice paths
//! ## Problem 15
//!
//! Starting in the top left corner of a 2×2 grid, and only being able to move
//! to the right and down, there are exactly 6 routes to the bottom right corner.
//!
//! ![illustration](https://projecteuler.net/project/images/p015.gif)
//!
//! How many such routes are there through a 20×20 grid?
//!
use ::num_bigint::BigUint;
use ::num_traits::one;

const GRID_SIZE: (usize, usize) = (20, 20);

fn factorial(n: usize) -> BigUint {
    (2usize..n+1).fold(one::<BigUint>(), |mut p, i| {
        p *= i;
        p
    })
}

fn binomial_coefficient(n: usize, k: usize) -> BigUint {
    factorial(n) / (factorial(k) * factorial(n - k))
}

pub fn p015() -> String {
    // The number of lattice paths from (0,0) to (n,k) is equal to the
    // binomial coefficient (n + k, k)
    binomial_coefficient(GRID_SIZE.0 + GRID_SIZE.1, GRID_SIZE.1).to_str_radix(10)
}