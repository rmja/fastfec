#![feature(portable_simd)]

pub mod catalog;
pub mod convolutional;
pub mod interleaver;
pub mod ratematching;
pub mod turbo;

pub type Llr = i8;

/// The fec code rate `k/n`. For every `k` input bits the coder generates a total of `n` bits.
pub struct CodeRate {
    /// The code rate numerator
    pub k: u8,
    /// The code rate denominator
    pub n: u8,
}
