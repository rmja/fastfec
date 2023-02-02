mod code;
#[cfg(feature = "alloc")]
mod decoder;
mod encoder;
mod symbol;

pub use code::TurboCode;
#[cfg(feature = "alloc")]
pub use decoder::TurboDecoder;
pub use encoder::{TurboEncoder, TurboEncoderOutputWriter};
pub use symbol::TurboSymbol;

pub mod umts {
    #[cfg(feature = "alloc")]
    pub use super::decoder::UmtsTurboDecoder;
}
