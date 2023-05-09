use crate::convolutional::{ConvolutionalCode, ConvolutionalCodeExt};

#[const_trait]
pub trait TurboCode: Default {
    type ConstituentEncoderCode: ConvolutionalCode;
    const TERMINATE_FIRST: bool;
    const TERMINATE_SECOND: bool;
}

pub(super) fn assert_consituent_encoder<C: TurboCode>() {
    let rate = C::ConstituentEncoderCode::rate();
    assert!(rate.k == 1);
    assert!(rate.n == 2);
    assert!(C::ConstituentEncoderCode::is_systematic());
}
