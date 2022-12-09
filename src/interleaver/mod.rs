use core::ops::Deref;
use alloc::vec;

pub mod qpp;
pub mod umts;

/// The interleaver.
pub trait Interleaver: IntoIterator<Item = InterleaverMapping> + Copy {
    /// The interleaver length.
    fn len(&self) -> usize;

    /// Interleave a buffer in place.
    fn interleave<T: Copy + Default + Sized>(&self, buffer: &mut [T]) {
        assert_eq!(self.len(), buffer.len());

        let mut interleaved = vec![T::default(); buffer.len()];

        for InterleaverMapping(i, ii) in self.into_iter() {
            interleaved[i] = buffer[ii];
        }

        buffer.copy_from_slice(&interleaved);
    }

    /// Deinterleave a buffer in place.
    fn deinterleave<T: Copy + Default + Sized>(&self, buffer: &mut [T]) {
        assert_eq!(self.len(), buffer.len());

        let mut deinterleaved = vec![T::default(); buffer.len()];

        for InterleaverMapping(i, ii) in self.into_iter() {
            deinterleaved[ii] = buffer[i];
        }

        buffer.copy_from_slice(&deinterleaved);
    }
}

/// Interleaver mapping.
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
