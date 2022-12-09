mod code;
mod encoder;
pub mod bcjr;

pub use code::ConvolutionalCode;
pub use encoder::ConvolutionalEncoder;

/// Concatenated output bits for each generator polynomial.
/// The output for the first polynomial maps to the least significant bit 0,
/// The next polynomial to bit 1, etc.
pub type EncoderOutput = usize;