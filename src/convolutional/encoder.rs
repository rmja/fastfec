use core::marker::PhantomData;

use super::{code::CodeState, ConvolutionalCode, ConvolutionalCodeExt, EncoderOutput};

#[derive(Default)]
pub struct ConvolutionalEncoder<C: ConvolutionalCode> {
    _code: PhantomData<C>,
    state: CodeState,
}

impl<C: ConvolutionalCode> ConvolutionalEncoder<C> {
    /// Get the next encoder output given `input`
    pub fn get_output(&mut self, input: bool) -> EncoderOutput {
        let output = C::get_output(self.state, input);
        self.state = C::get_next_state(self.state, input);
        output
    }

    /// Get the next encoder termination
    pub fn get_termination_output(&mut self) -> EncoderOutput {
        let input = C::get_termination_input(self.state);
        let output = C::get_output(self.state, input);
        self.state = C::get_next_state(self.state, input);
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        catalog,
        convolutional::{ConvolutionalCodeExt, EncoderOutput},
    };

    use super::ConvolutionalEncoder;

    #[test]
    #[rustfmt::skip]
    fn can_encode_umts() {
        can_encode_umts_case(
            &[
                // 0xBA
                // A: Systematic output
                // B: Parity output
                0b11, 0b01, 0b00, 0b11, 0b11, 0b00, 0b10, 0b11,
                0b00, 0b00, 0b00,
            ],
            &[
                1, 1, 0, 1, 1, 0, 0, 1
            ],
        );

        can_encode_umts_case(
            &[
                0b00, 0b11, 0b01, 0b11, 0b01, 0b10, 0b11, 0b01,
                0b10, 0b11, 0b10, 0b00, 0b01, 0b11, 0b01, 0b10,
                0b11, 0b10, 0b11,
            ],
            &[
                0, 1, 1, 1, 1, 0, 1, 1,
                0, 1, 0, 0, 1, 1, 1, 0,
            ],
        );

        can_encode_umts_case(
            &[
                0b00, 0b00, 0b11, 0b10, 0b01, 0b11, 0b00, 0b11,
                0b10, 0b10, 0b01, 0b01, 0b11, 0b00, 0b00, 0b11,
                0b01, 0b00, 0b00, 0b01, 0b10, 0b11, 0b01, 0b01,
                0b01, 0b11, 0b00,
            ],
            &[
                0, 0, 1, 0, 1, 1, 0, 1,
                0, 0, 1, 1, 1, 0, 0, 1,
                1, 0, 0, 1, 0, 1, 1, 1,
            ],
        );

        can_encode_umts_case(
            &[
                0b11, 0b01, 0b11, 0b01, 0b10, 0b11, 0b10, 0b00,
                0b01, 0b00, 0b00, 0b01, 0b01, 0b10, 0b01, 0b10,
                0b01, 0b01, 0b00, 0b01, 0b11, 0b01, 0b01, 0b10,
                0b10, 0b00, 0b11, 0b11, 0b00, 0b01, 0b01, 0b01,
                0b11, 0b00, 0b00,
            ],
            &[
                1, 1, 1, 1, 0, 1, 0, 0,
                1, 0, 0, 1, 1, 0, 1, 0,
                1, 1, 0, 1, 1, 1, 1, 0,
                0, 0, 1, 1, 0, 1, 1, 1,
            ],
        );
    }

    fn can_encode_umts_case(expected: &[EncoderOutput], input: &[u8]) {
        // Given
        let input: Vec<bool> = input.into_iter().map(|b| *b == 1).collect();
        let mut encoder = ConvolutionalEncoder::<crate::catalog::UMTS>::default();

        let mut output = Vec::new();

        // When
        for bit in input {
            output.push(encoder.get_output(bit));
        }

        for _ in 0..catalog::UMTS::mem() {
            output.push(encoder.get_termination_output());
        }

        // Then
        assert_eq!(expected, output);
    }
}
