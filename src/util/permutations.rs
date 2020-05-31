pub struct Permutations<'a, T: 'a>(&'a mut [T], bool);

#[allow(clippy::should_implement_trait)]
impl<'a, T: Ord> Permutations<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Permutations<'a, T> {
        assert!(!slice.is_empty());
        slice.sort();
        Permutations(slice, true)
    }

    pub fn next(&mut self) -> Option<&[T]> {
        if self.1 {
            self.1 = false;
            return Some(self.0);
        }

        // Find non-increasing suffix
        let mut i = self.0.len() - 1;
        while i > 0 && self.0[i - 1] >= self.0[i] {
            i -= 1;
        }

        if i == 0 {
            return None;
        }

        // Find successor to pivot
        let mut j = self.0.len() - 1;
        while self.0[j] <= self.0[i - 1] {
            j -= 1;
        }

        self.0.swap(i - 1, j);

        // Reverse suffix
        self.0[i..].reverse();
        Some(self.0)
    }
}