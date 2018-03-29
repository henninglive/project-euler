//! # Largest palindrome product
//! ## Problem 4
//!
//! A palindromic number reads the same both ways. The largest palindrome made
//! from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.
//!

fn is_palindrome_base_10(n: usize) -> bool {
    let s = n.to_string();
    let sb = s.as_bytes();

    let mut a = 0;
    let mut b = sb.len() - 1;

    loop {
        if sb[a] != sb[b] {
            return false;
        }

        if b == 0 {
            return true;
        }

        a += 1;
        b -= 1;
    }
}

pub fn p4() -> String {
    let mut max = 0;
    for a in (100..1000).rev() {
        for b in (100..1000).rev() {
            let p = a * b;
            if p <= max {
                continue;
            }

            if is_palindrome_base_10(p) {
                max = p;
            }
        }
    }
    max.to_string()
}