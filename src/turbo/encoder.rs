use core::marker::PhantomData;

use crate::{
    convolutional::{ConvolutionalCodeExt, ConvolutionalEncoder, EncoderOutput},
    interleaver::{Interleaver, InterleaverMapping},
    turbo::code::assert_consituent_encoder,
    BitView,
};

use super::TurboCode;

pub struct TurboEncoder<C: TurboCode> {
    _code: PhantomData<C>,
}

pub trait TurboEncoderOutputWriter {
    /// Write output bits.
    ///
    /// Bit 0 corresponds to the systematic bit,
    /// bit 1 to the output of the first constituent encoder, and
    /// bit 2 to the output of the second constituent encoder.
    ///
    /// # Arguments
    ///
    /// * `output` - The encoder output.
    fn write_output(&mut self, output: EncoderOutput);

    /// Write termination bits.
    ///
    /// Bit 0 corresponds to the termination input and
    /// bit 1 to the corresponding parity output.
    ///
    /// # Arguments
    ///
    /// * `encoder_index` - The index of the constituent encoder for which the termination is generated.
    /// * `output` - The termination output.
    fn write_termination_output(&mut self, encoder_index: usize, output: EncoderOutput);
}

impl<C: TurboCode> TurboEncoder<C> {
    pub fn new() -> Self {
        assert_consituent_encoder::<C>();
        Self { _code: PhantomData }
    }

    pub fn encode<S, I, W>(&self, source: S, interleaver: &I, writer: &mut W)
    where
        S: BitView,
        I: Interleaver,
        W: TurboEncoderOutputWriter,
    {
        assert_eq!(source.len2(), interleaver.len());

        let mut first_encoder = ConvolutionalEncoder::<C::ConstituentEncoderCode>::default();
        let mut second_encoder = ConvolutionalEncoder::<C::ConstituentEncoderCode>::default();

        for InterleaverMapping(i, ii) in interleaver.iter() {
            let input = source.get(i);
            let first_output = first_encoder.get_output(input);
            let second_output = second_encoder.get_output(source.get(ii));

            // Write the parity and systematic output from the first encoder and the parity from the second encoder.
            writer.write_output(first_output | (second_output & 0x02) << 1);
        }

        if C::TERMINATE_FIRST || C::TERMINATE_SECOND {
            for _ in 0..C::ConstituentEncoderCode::mem() {
                if C::TERMINATE_FIRST {
                    let first_output = first_encoder.get_termination_output();
                    writer.write_termination_output(0, first_output);
                }

                if C::TERMINATE_SECOND {
                    let second_output = second_encoder.get_termination_output();
                    writer.write_termination_output(1, second_output);
                }
            }
        }
    }
}

impl<C: TurboCode> Default for TurboEncoder<C> {
    fn default() -> Self {
        TurboEncoder::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{catalog, convolutional::EncoderOutput, interleaver::qpp::QppInterleaver};

    use super::*;
    use bitvec::prelude::*;

    #[test]
    fn can_encode_from_boolslice() {
        let input = [false; 8].as_ref();

        let encoder = TurboEncoder::<catalog::UMTS>::default();
        let interleaver = QppInterleaver::new(input.len(), 3, 0);
        let mut writer = TurboEncoderOutputWriterStub::new();

        encoder.encode(input, &interleaver, &mut writer);
    }

    #[test]
    fn can_encode_from_fixedboolslice() {
        let input = [false; 8];

        let encoder = TurboEncoder::<catalog::UMTS>::default();
        let interleaver = QppInterleaver::new(input.len(), 3, 0);
        let mut writer = TurboEncoderOutputWriterStub::new();

        encoder.encode(&input, &interleaver, &mut writer);
    }

    #[test]
    fn can_encode_from_bitslice() {
        let input = &[0x00u8];
        let bitslice = input.view_bits::<Msb0>();
        assert_eq!(8, bitslice.len());

        let encoder = TurboEncoder::<catalog::UMTS>::default();
        let interleaver = QppInterleaver::new(bitslice.len(), 3, 0);
        let mut writer = TurboEncoderOutputWriterStub::new();

        encoder.encode(bitslice, &interleaver, &mut writer);
    }

    #[test]
    #[rustfmt::skip]
    fn can_encode() {
        can_encode_case(&[
            // 0bCBA
            // A: Systematic output from first encoder
            // B: Parity output from first encoder
            // C: Parity output from second encoder
            0b111, 0b001, 0b000, 0b111, 0b111, 0b100, 0b010, 0b111,
            // 0bBA, 0bDC
            // A(C): First (second) encoder termination input
            // B(D): First (second) encoder parity output
            0b00, 0b11,
            0b00, 0b00,
            0b00, 0b00,
        ], &[
            1, 1, 0, 1, 1, 0, 0, 1,
        ], 3, 0);

        can_encode_case(&[
            0b000, 0b011, 0b101, 0b011, 0b101, 0b010, 0b011, 0b101,
            0b110, 0b111, 0b110, 0b000, 0b001, 0b111, 0b001, 0b010,
            0b11, 0b01,
            0b10, 0b11,
            0b11, 0b00,
        ], &[
            0, 1, 1, 1, 1, 0, 1, 1,
            0, 1, 0, 0, 1, 1, 1, 0,
        ], 1, 4);

        can_encode_case(&[
            0b000, 0b100, 0b011, 0b010, 0b101, 0b111, 0b000, 0b111,
            0b010, 0b010, 0b101, 0b101, 0b111, 0b100, 0b100, 0b011,
            0b101, 0b100, 0b100, 0b101, 0b110, 0b011, 0b101, 0b101,
            0b01, 0b00,
            0b11, 0b10,
            0b00, 0b11,
        ], &[
            0, 0, 1, 0, 1, 1, 0, 1,
            0, 0, 1, 1, 1, 0, 0, 1,
            1, 0, 0, 1, 0, 1, 1, 1,
        ], 1, 6);

        can_encode_case(&[
            0b111, 0b001, 0b011, 0b001, 0b110, 0b011, 0b010, 0b100,
            0b001, 0b100, 0b100, 0b101, 0b101, 0b110, 0b001, 0b110,
            0b001, 0b001, 0b000, 0b001, 0b011, 0b001, 0b001, 0b010,
            0b110, 0b100, 0b111, 0b111, 0b000, 0b101, 0b001, 0b101,
            0b11, 0b10,
            0b00, 0b11,
            0b00, 0b00,
        ], &[
            1, 1, 1, 1, 0, 1, 0, 0,
            1, 0, 0, 1, 1, 0, 1, 0,
            1, 1, 0, 1, 1, 1, 1, 0,
            0, 0, 1, 1, 0, 1, 1, 1,
        ], 1, 4);
    }

    fn can_encode_case(expected: &[EncoderOutput], input: &[u8], f1: u16, f2: u16) {
        // Given
        let input: Vec<bool> = input.into_iter().map(|b| *b == 1).collect();
        let encoder = TurboEncoder::<catalog::UMTS>::default();
        let interleaver = QppInterleaver::new(input.len(), f1, f2);
        let mut writer = TurboEncoderOutputWriterStub::new();

        // When
        encoder.encode(&input[..], &interleaver, &mut writer);

        // Then
        assert_eq!(3 * expected.len() - 6, writer.written_bit_count);
        assert_eq!(expected, writer.written);
    }

    struct TurboEncoderOutputWriterStub {
        written: Vec<EncoderOutput>,
        written_bit_count: usize,
    }

    impl TurboEncoderOutputWriterStub {
        pub fn new() -> Self {
            Self {
                written: Vec::new(),
                written_bit_count: 0,
            }
        }
    }

    impl TurboEncoderOutputWriter for TurboEncoderOutputWriterStub {
        fn write_output(&mut self, output: EncoderOutput) {
            self.written.push(output);
            self.written_bit_count += 3;
        }

        fn write_termination_output(&mut self, _encoder_index: usize, output: EncoderOutput) {
            self.written.push(output);
            self.written_bit_count += 2;
        }
    }
}
