#[cfg(feature = "alloc")]
pub mod bcjr;
mod code;
mod encoder;

pub use code::{ConvolutionalCode, ConvolutionalCodeExt};
pub use encoder::ConvolutionalEncoder;

/// Concatenated output bits for each generator polynomial.
/// The output for the first polynomial maps to the least significant bit 0,
/// The next polynomial to bit 1, etc.
pub type EncoderOutput = usize;
