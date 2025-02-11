#![cfg_attr(not(test), no_std)]
#![feature(const_trait_impl)]
#![feature(portable_simd)]

mod bitview;
pub mod catalog;
pub mod convolutional;
pub mod interleaver;
pub mod ratematching;
pub mod turbo;

pub type Llr = i8;

pub trait LlrMul {
    fn mul(self, rhs: Llr) -> Llr;
}

impl LlrMul for bool {
    fn mul(self, rhs: Llr) -> Llr {
        if self {
            rhs
        } else {
            -rhs
        }
    }
}

/// The fec code rate `k/n`. For every `k` input bits the coder generates a total of `n` bits.
pub struct CodeRate {
    /// The code rate numerator
    pub k: u8,
    /// The code rate denominator
    pub n: u8,
}

pub use bitview::BitView;
