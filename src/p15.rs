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

use ::util::Factorize;

const GRID_SIZE: (usize, usize) = (20, 20);

#[derive(Debug)]
struct Product(Vec<usize>);

#[derive(Debug)]
struct Fraction(Product, Product);

impl Product {
    fn factorial(n: usize) -> Product {
        let mut v = (1..n + 1)
            .flat_map(|i| Factorize::new(i))
            .collect::<Vec<_>>();
        v.sort();
        Product(v)
    }

    fn compute(&self) -> usize {
        self.0.iter().product()
    }
}

impl ::std::ops::Mul for Product {
    type Output = Product;
    fn mul(mut self, other: Product) -> Product {
        self.0.extend(other.0);
        self.0.sort();
        self
    }
}

impl Fraction {
    fn compute(&self) -> usize {
        self.0.compute() / self.1.compute()
    }
}

impl ::std::ops::Div for Product {
    type Output = Fraction;
    fn div(mut self, mut other: Product) -> Fraction {
        let mut i = 0;
        let mut len = other.0.len();
        while i < len {
            match self.0.iter().position(|e| *e == other.0[i]) {
                Some(j) => {
                    other.0.remove(i);
                    self.0.remove(j);
                    len -= 1;
                }
                _ => i += 1,
            }

        }
        Fraction(self, other)
    }
}

fn binomial_coefficient(n: usize, k: usize) -> usize {
    // The numbers we are working with quite large, so we factorize the terms,
    // this allows us to simplify the fraction before we calculate the result.

    ( Product::factorial(n) /
        (Product::factorial(k) * Product::factorial(n - k))
    ).compute()
}

pub fn p15() -> String {
    // The number of lattice paths from (0,0) to (n,k) is equal to the
    // binomial coefficient (n + k, k)

    binomial_coefficient(GRID_SIZE.0 + GRID_SIZE.1, GRID_SIZE.1)
        .to_string()
}