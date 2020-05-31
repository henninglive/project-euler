use num::Integer;
use std::mem;

#[derive(Debug)]
pub struct Fibonacci<I: Integer>(I, I);

impl<I: Integer + Clone> Iterator for Fibonacci<I> {
    type Item = I;
    fn next(&mut self) -> Option<I> {
        let mut tmp = self.1.clone() + self.0.clone();
        mem::swap(&mut tmp, &mut self.1);
        mem::swap(&mut tmp, &mut self.0);
        Some(tmp)
    }
}

impl<I: Integer> Fibonacci<I> {
    pub fn one() -> Fibonacci<I> {
        Fibonacci(I::one(), I::one())
    }
}

#[test]
fn test_fibonacci() {
    assert_eq!(
        &[1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144],
        &Fibonacci::<u32>::one().take(12).collect::<Vec<_>>()[..]
    );
}
