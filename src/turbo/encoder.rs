use crate::{
    convolutional::{ConvolutionalEncoder, EncoderOutput},
    interleaver::Interleaver,
};

use super::TurboCode;

pub struct TurboEncoder {
    code: TurboCode,
}

pub trait TurboEncoderOutputWriter {
    /// Write output bits.
    ///
    /// # Arguments
    ///
    /// * `output` - The encoder output. Bit 0 corresponds to systematic bit, bit 1 to the output of the first constituent encoder and bit 2 to the output of the second constituent encoder.
    fn write_output(&mut self, output: EncoderOutput);

    /// Write termination bits.
    ///
    /// # Arguments
    ///
    /// * `encoder_index` - The index of the constituent encoder for which the termination is generated.
    /// * `output` - The termination output. Bit 0 corresponds to the termination input and bit 1 to the corresponding parity output.
    fn write_termination_output(&mut self, encoder_index: usize, output: EncoderOutput);
}

impl TurboEncoder {
    pub const fn new(code: TurboCode) -> Self {
        Self { code }
    }

    pub fn encode<I: Interleaver, W: TurboEncoderOutputWriter>(
        &self,
        source: &[bool],
        interleaver: I,
        writer: &mut W,
    ) {
        assert_eq!(source.len(), interleaver.len());

        let mut first_encoder = ConvolutionalEncoder::new(self.code.constituent_encoder_code);
        let mut second_encoder = ConvolutionalEncoder::new(self.code.constituent_encoder_code);

        for (input, ii) in source.into_iter().zip(interleaver) {
            let first_output = first_encoder.get_output(*input);
            let second_output = second_encoder.get_output(source[*ii]);

            // Write the parity and systematic output from the first encoder and the parity from the second encoder.
            writer.write_output(first_output | (second_output & 0x02) << 1);
        }

        if self.code.terminate_first || self.code.terminate_second {
            for _ in 0..self.code.constituent_encoder_code.mem() {
                if self.code.terminate_first {
                    let first_output = first_encoder.get_termination_output();
                    writer.write_termination_output(0, first_output);
                }

                if self.code.terminate_second {
                    let second_output = second_encoder.get_termination_output();
                    writer.write_termination_output(1, second_output);
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{convolutional::EncoderOutput, interleaver::qpp::QppInterleaver};

    use super::*;

    #[test]
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
        let encoder = TurboEncoder::new(crate::catalog::UMTS);
        let interleaver = QppInterleaver::new(input.len(), f1, f2);
        let mut writer = TurboEncoderOutputWriterStub {
            written: Vec::new(),
            written_bit_count: 0,
        };

        // When
        encoder.encode(&input, interleaver, &mut writer);

        // Then
        assert_eq!(3 * expected.len() - 6, writer.written_bit_count);
        assert_eq!(expected, writer.written);
    }

    struct TurboEncoderOutputWriterStub {
        written: Vec<EncoderOutput>,
        written_bit_count: usize,
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
