//! **Problem 31**: Coin sums
//!
//! In England the currency is made up of pound, £, and pence, p, and there
//! are eight coins in general circulation:
//!
//! `1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p)`.
//!
//! It is possible to make £2 in the following way:
//!
//! `1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p`
//!
//! How many different ways can £2 be made using any number of coins?
//!
//! [Problem 31 on projecteuler.net](https://projecteuler.net/problem=31)
//!

/// Array of the coin denominations
pub static COINS: [usize; 8] = [200, 100, 50, 20, 10, 5, 2, 1];

const TARGET: usize = 200;

fn recursive(mut value: usize, lvl: usize, count: &mut usize) {
    let coin = COINS[lvl];
    while value < TARGET {
        if lvl < COINS.len() - 1 {
            recursive(value, lvl + 1, count);
        }

        value += coin;

        if value == TARGET {
            *count += 1;
        }
    }
}

/// Calculate solution to Problem 31
pub fn solution() -> String {
    let mut count = 0;
    recursive(0, 0, &mut count);
    count.to_string()
}

#[test]
fn test_solution() {
    assert_eq!("73682", solution());
}
