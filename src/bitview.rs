use bitvec::prelude::*;

pub trait BitView {
    fn get(&self, index: usize) -> bool;
    fn len2(&self) -> usize;
}

impl BitView for &[bool] {
    fn get(&self, index: usize) -> bool {
        self[index]
    }

    fn len2(&self) -> usize {
        self.len()
    }
}

impl<const N: usize> BitView for &[bool; N] {
    fn get(&self, index: usize) -> bool {
        self[index]
    }

    fn len2(&self) -> usize {
        self.len()
    }
}

impl<T: BitStore, O: BitOrder> BitView for &BitSlice<T, O> {
    fn get(&self, index: usize) -> bool {
        self[index]
    }

    fn len2(&self) -> usize {
        self.len()
    }
}
