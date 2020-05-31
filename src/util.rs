
use num::Integer;

#[derive(Debug)]
pub struct Fib<I: Integer>(I, I);

impl<I: Integer + Clone> Iterator for Fib<I> {
    type Item = I;
    fn next(&mut self) -> Option<I> {
        let mut tmp = self.1.clone() + self.0.clone();
        std::mem::swap(&mut tmp, &mut self.1);
        std::mem::swap(&mut tmp, &mut self.0);
        Some(tmp)
    }
}

impl<I: Integer> Fib<I> {
    /*
    pub fn zero() -> Fib<I> {
        Fib(I::zero(), I::one())
    }
    */

    pub fn one() -> Fib<I> {
        Fib(I::one(), I::one())
    }
}

#[derive(Debug, Clone)]
pub struct Factorize(usize);

impl Factorize {
    pub fn new(n: usize) -> Factorize {
        Factorize(n)
    }
}

impl Iterator for Factorize {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.0 <= 1 {
            None
        } else if self.0 % 2 == 0 {
            self.0 /= 2;
            Some(2)
        } else {
            let mut f = 3;
            loop {
                if self.0 % f == 0 {
                    self.0 /= f;
                    return Some(f)
                }
                f += 2;
            }
        }
    }
}

#[test]
fn test_factorize() {
    assert_eq!(&[2, 2, 2, 3, 3, 5, 7],
        &Factorize::new(2520).collect::<Vec<_>>()[..]);
}