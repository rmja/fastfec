use crate::{convolutional::ConvolutionalCode, turbo::TurboCode};

/// The abrantes code defined by the transfer polynomial G = [1, (1+x^2)/(1+x+x^2)]
/// http://paginas.fe.up.pt/~sam/textos/From%20BCJR%20to%20turbo.pdf
/// The outgoing lines types from state 2 and 3 in Fig. 2, 3, 4b, 5, 11, 12, 13, 14, 15 are incorrect. Solid and dashed is swapped.
/// The correct transitions are in Fig. 16.
#[derive(Default)]
pub struct ABRANTES;

impl ConvolutionalCode for ABRANTES {
    const CONSTRAINT_LENGTH: usize = 3;

    const GENERATORS: &'static [usize] = &[
        0b111, // Systematic part
        0b101, // Encoded part
    ];

    const FEEDBACK: usize = 0b111;
}

/// The UMTS/LTE Turbo Code defined by the transfer polynomial G = [1, (1+x+x^3)/(1+x^2+x^3)]
#[derive(Default)]
pub struct UMTS;

impl ConvolutionalCode for UMTS {
    const CONSTRAINT_LENGTH: usize = 4;

    const GENERATORS: &'static [usize] = &[
        0b1011, // Systematic part (should match the feedback http://matrix.etseq.urv.es/manuals/matlab/toolbox/comm/tutor124.html)
        0b1101, // Encoded part
    ];

    const FEEDBACK: usize = 0b1011;
}

impl TurboCode for UMTS {
    type ConstituentEncoderCode = UMTS;
    const TERMINATE_FIRST: bool = true;
    const TERMINATE_SECOND: bool = true;
}

/// The Mioty code per §6.4.6.3
#[derive(Default)]
pub struct MIOTY;

impl ConvolutionalCode for MIOTY {
    const CONSTRAINT_LENGTH: usize = 7;

    const GENERATORS: &'static [usize] = &[
        0b1101101, // 0x6D
        0b1010011, // 0x53
        0b1011111, // 0x5F
    ];

    const FEEDBACK: usize = 0;
}
