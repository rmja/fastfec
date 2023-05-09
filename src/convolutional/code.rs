use crate::CodeRate;

use super::EncoderOutput;

pub(crate) type CodeState = usize;

#[const_trait]
pub trait ConvolutionalCode: Default {
    /// The constraint length of the code, i.e.
    /// the number of bits stored in each shift register, including the current input bit.
    const CONSTRAINT_LENGTH: usize;

    /// The generator polynomials descibing each output path.
    /// The code has rate 1/k where `k` is the number of generator polynomials.
    const GENERATORS: &'static [usize];

    /// The feedback polynomial describing the feedback path.
    const FEEDBACK: usize;
}

pub trait ConvolutionalCodeExt<C: ConvolutionalCode> {
    /// The code rate.
    fn rate() -> CodeRate {
        CodeRate {
            k: 1,
            n: C::GENERATORS.len() as u8,
        }
    }

    /// The memory in the code, i.e. the number of shift registers.
    fn mem() -> usize {
        C::CONSTRAINT_LENGTH - 1
    }

    /// Get whether the code is systematic.
    /// The code is systematic if the first generator polynimal equals the feedback polynomial.
    fn is_systematic() -> bool {
        C::GENERATORS[0] == C::FEEDBACK
    }

    fn get_next_state(current: CodeState, input: bool) -> CodeState {
        // Find the feedback sum
        let sum = (current & C::FEEDBACK).count_ones() as usize & 1;

        // Shift out the oldest value
        let mut next: CodeState = current >> 1;

        // Add the input + feedback as the newest value
        next |= (input as usize ^ sum) << (C::mem() - 1);

        next
    }

    /// Find the input that produces a 0 on the output of the first delay
    /// when used to compute the next state
    fn get_termination_input(current: CodeState) -> bool {
        // Find the input bit that produces a 0 after the first delay
        // Try with a guess of input=0
        let next_if_0 = C::get_next_state(current, false);

        // Take the MSB which is the newest value
        // If this value is 0 then our guess was right - the input should be zero.
        // If this value is 1 then it means that our guess was wrong, and we need to input 1 instead.
        // For both cases this conviently means that we can simply take this bit as our input.
        let input = next_if_0 >> (C::mem() - 1);

        input != 0
    }

    /// Get the output bits for all generator polynomials concatenated.
    /// E.g. for a rate 1/3 decoder there are three bits.
    /// The first polynomial gets the _least_ significant bit, the last polynomial gets the most significant bit significant bit,
    /// i.e. polynomial[0] -> bit 0
    fn get_output(current: CodeState, input: bool) -> EncoderOutput {
        let mut output = 0;
        let mut mask = 1;

        // Find the feedback sum
        let feedback_sum = (current & C::FEEDBACK).count_ones() as usize + input as usize;

        // Get the individual output for each generator and append to the final output
        for poly in C::GENERATORS {
            let sum = (feedback_sum + (current & poly).count_ones() as usize) & 1;
            output |= sum * mask;
            mask <<= 1;
        }

        output
    }
}

impl<C: ConvolutionalCode> ConvolutionalCodeExt<C> for C {}

#[cfg(test)]
mod tests {
    use crate::catalog;

    use super::ConvolutionalCodeExt;

    #[test]
    fn abrantes() {
        // Given
        type Code = catalog::ABRANTES;

        // When

        // Then
        assert_eq!(0, Code::get_next_state(0, false));
        assert_eq!(2, Code::get_next_state(0, true));
        assert_eq!(2, Code::get_next_state(1, false));
        assert_eq!(0, Code::get_next_state(1, true));
        assert_eq!(3, Code::get_next_state(2, false));
        assert_eq!(1, Code::get_next_state(2, true));
        assert_eq!(1, Code::get_next_state(3, false));
        assert_eq!(3, Code::get_next_state(3, true));

        assert_eq!(false, Code::get_termination_input(0));
        assert_eq!(true, Code::get_termination_input(1));
        assert_eq!(true, Code::get_termination_input(2));
        assert_eq!(false, Code::get_termination_input(3));

        assert_eq!(0b00, Code::get_output(0, false));
        assert_eq!(0b11, Code::get_output(0, true));
        assert_eq!(0b00, Code::get_output(1, false));
        assert_eq!(0b11, Code::get_output(1, true));
        assert_eq!(0b10, Code::get_output(2, false));
        assert_eq!(0b01, Code::get_output(2, true));
        assert_eq!(0b10, Code::get_output(3, false));
        assert_eq!(0b01, Code::get_output(3, true));
    }

    #[test]
    fn umts() {
        // Given
        type Code = catalog::UMTS;

        // When
        assert_eq!(0, Code::get_next_state(0, false));
        assert_eq!(4, Code::get_next_state(0, true));
        assert_eq!(4, Code::get_next_state(1, false));
        assert_eq!(0, Code::get_next_state(1, true));
        assert_eq!(5, Code::get_next_state(2, false));
        assert_eq!(1, Code::get_next_state(2, true));
        assert_eq!(1, Code::get_next_state(3, false));
        assert_eq!(5, Code::get_next_state(3, true));
        assert_eq!(2, Code::get_next_state(4, false));
        assert_eq!(6, Code::get_next_state(4, true));
        assert_eq!(6, Code::get_next_state(5, false));
        assert_eq!(2, Code::get_next_state(5, true));
        assert_eq!(7, Code::get_next_state(6, false));
        assert_eq!(3, Code::get_next_state(6, true));
        assert_eq!(3, Code::get_next_state(7, false));
        assert_eq!(7, Code::get_next_state(7, true));

        assert_eq!(false, Code::get_termination_input(0));
        assert_eq!(true, Code::get_termination_input(1));
        assert_eq!(true, Code::get_termination_input(2));
        assert_eq!(false, Code::get_termination_input(3));
        assert_eq!(false, Code::get_termination_input(4));
        assert_eq!(true, Code::get_termination_input(5));
        assert_eq!(true, Code::get_termination_input(6));
        assert_eq!(false, Code::get_termination_input(7));

        assert_eq!(0b00, Code::get_output(0, false));
        assert_eq!(0b11, Code::get_output(0, true));
        assert_eq!(0b00, Code::get_output(1, false));
        assert_eq!(0b11, Code::get_output(1, true));
        assert_eq!(0b10, Code::get_output(2, false));
        assert_eq!(0b01, Code::get_output(2, true));
        assert_eq!(0b10, Code::get_output(3, false));
        assert_eq!(0b01, Code::get_output(3, true));
        assert_eq!(0b10, Code::get_output(4, false));
        assert_eq!(0b01, Code::get_output(4, true));
        assert_eq!(0b10, Code::get_output(5, false));
        assert_eq!(0b01, Code::get_output(5, true));
        assert_eq!(0b00, Code::get_output(6, false));
        assert_eq!(0b11, Code::get_output(6, true));
        assert_eq!(0b00, Code::get_output(7, false));
        assert_eq!(0b11, Code::get_output(7, true));
    }
}
