use super::{Interleaver, InterleaverMapping};

/// Quadratic Polynomial Permutation (QPP) Interleaver.
/// Permutation is computed using the formula:
///    pi(i) = (f1 * i + f2 * i^2) mod k.
/// The equation can be rewritten as the following recursive expression:
///    pi(i+1) = (pi(i) + g(i)) mod k,
/// where
///    g(i+1) = (g(i) + (2f2 mod k)) mod k,
/// with `2f2 mod k` being constant for each iteration.
#[derive(Clone, Copy)]
pub struct QppInterleaver {
    /// The block length `k` in bits.
    length: usize,
    f1: u16,
    f2: u16,
}

impl QppInterleaver {
    /// Create a new interleaver
    pub const fn new(length: usize, f1: u16, f2: u16) -> Self {
        Self { length, f1, f2 }
    }
}

impl Interleaver for QppInterleaver {
    fn len(&self) -> usize {
        self.length
    }

    fn get(&self, i: usize) -> usize {
        let i = u64::try_from(i).unwrap();
        let f1 = u64::try_from(self.f1).unwrap();
        let f2 = u64::try_from(self.f2).unwrap();
        let length = u64::try_from(self.length).unwrap();

        ((f1 * i + f2 * i * i) % length).try_into().unwrap()
    }

    fn iter(&self) -> impl Iterator<Item = InterleaverMapping> {
        QppIterator {
            length: self.length,
            incr: (2 * self.f2 as usize) % self.length,
            pi: 0,
            g: (self.f1 as usize + self.f2 as usize) % self.length,
            i: 0,
        }
    }
}

pub struct QppIterator {
    length: usize,
    incr: usize,
    pi: usize,
    g: usize,
    i: usize,
}

impl Iterator for QppIterator {
    type Item = InterleaverMapping;

    fn next(&mut self) -> Option<InterleaverMapping> {
        if self.i < self.length {
            let pi = self.pi;
            let g = self.g;
            let i = self.i;

            self.pi = (pi + g) % self.length;
            self.g = (g + self.incr) % self.length;
            self.i = i + 1;

            Some(InterleaverMapping(i, pi))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }
}

impl ExactSizeIterator for QppIterator {}
