use alloc::vec;
use core::ops::Deref;

pub mod qpp;
pub mod umts;

/// The interleaver.
#[allow(clippy::len_without_is_empty)]
pub trait Interleaver {
    /// The interleaver length.
    fn len(&self) -> usize;

    /// Get the interleaved index.
    /// It is slower to call this function `k` times than iterating the entire
    /// permuted sequence.
    fn get(&self, i: usize) -> usize;

    /// Get an iterator that produces the permuted sequence.
    /// It produces `k` permutations and is faster than invoking `pi` `k` times.
    fn iter(&self) -> impl Iterator<Item = InterleaverMapping>;

    /// Interleave a buffer in place.
    fn interleave<T: Copy + Default + Sized>(&self, buffer: &mut [T]) {
        assert_eq!(self.len(), buffer.len());

        let mut interleaved = vec![T::default(); buffer.len()];

        for InterleaverMapping(i, ii) in self.iter() {
            interleaved[i] = buffer[ii];
        }

        buffer.copy_from_slice(&interleaved);
    }

    /// Deinterleave a buffer in place.
    fn deinterleave<T: Copy + Default + Sized>(&self, buffer: &mut [T]) {
        assert_eq!(self.len(), buffer.len());

        let mut deinterleaved = vec![T::default(); buffer.len()];

        for InterleaverMapping(i, ii) in self.iter() {
            deinterleaved[ii] = buffer[i];
        }

        buffer.copy_from_slice(&deinterleaved);
    }
}

/// Interleaver mapping.
#[derive(Clone, Copy)]
pub struct InterleaverMapping(
    /// The original index
    pub usize,
    /// The inteleaved index
    pub usize,
);

impl Deref for InterleaverMapping {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.1 // Emit the interleaved index
    }
}
