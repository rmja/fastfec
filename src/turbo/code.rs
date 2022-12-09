use crate::convolutional::ConvolutionalCode;

#[derive(Clone, Copy)]
pub struct TurboCode {
    pub constituent_encoder_code: ConvolutionalCode,
    pub terminate_first: bool,
    pub terminate_second: bool,
}

impl TurboCode {
    pub const fn new(constituent_encoder_code: ConvolutionalCode) -> Self {
        let rate = constituent_encoder_code.rate();
        assert!(rate.k == 1);
        assert!(rate.n == 2);
        assert!(constituent_encoder_code.is_systematic());

        Self {
            constituent_encoder_code,
            terminate_first: true,
            terminate_second: true,
        }
    }
}
