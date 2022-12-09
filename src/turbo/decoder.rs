use std::marker::PhantomData;

use crate::{
    convolutional::bcjr::{umts::UmtsState, BcjrDecoder, BcjrSymbol, BcjrState},
    interleaver::{Interleaver, InterleaverMapping},
    Llr,
};

use super::{TurboCode, TurboSymbol};

pub struct TurboDecoder<S: BcjrState> {
    code: TurboCode,
    _state: PhantomData<S>,
}

pub type UmtsTurboDecoder = TurboDecoder<UmtsState>;

impl<S: BcjrState> TurboDecoder<S> {
    pub const fn new(code: TurboCode) -> Self {
        Self {
            code,
            _state: PhantomData,
        }
    }

    pub fn code(&self) -> TurboCode {
        self.code
    }

    pub fn decode<'a, I: Interleaver>(
        &'a self,
        input: &[TurboSymbol],
        interleaver: I,
        first_termination: &[BcjrSymbol],
        second_termination: &[BcjrSymbol],
    ) -> TurboDecoding<S, I> {
        // Prepare input for the first decoder
        let mut first_input = Vec::with_capacity(input.len() + first_termination.len());
        for symbol in input {
            first_input.push(BcjrSymbol::new(symbol.systematic, symbol.first_parity))
        }
        first_input.extend_from_slice(&first_termination);

        // Prepare input for the second decoder
        let mut second_input = Vec::with_capacity(input.len() + second_termination.len());
        for InterleaverMapping(i, ii) in interleaver {
            second_input.push(BcjrSymbol::new(
                input[ii].systematic,
                input[i].second_parity,
            ));
        }
        second_input.extend_from_slice(&second_termination);

        // Create a result buffer that the individual decoders can use
        let bcjr_result =
            vec![0; input.len() + usize::max(first_termination.len(), second_termination.len())];

        TurboDecoding {
            first_bcjr: BcjrDecoder::<S>::new(
                self.code.constituent_encoder_code,
                self.code.terminate_first,
            ),
            second_bcjr: BcjrDecoder::<S>::new(
                self.code.constituent_encoder_code,
                self.code.terminate_second,
            ),
            first_input,
            second_input,
            interleaver,
            bcjr_result,
            input_len: input.len(),
        }
    }
}

pub struct TurboDecoding<S: BcjrState, I: Interleaver> {
    first_bcjr: BcjrDecoder<S>,
    second_bcjr: BcjrDecoder<S>,
    interleaver: I,
    first_input: Vec<BcjrSymbol>,
    second_input: Vec<BcjrSymbol>,
    bcjr_result: Vec<Llr>,
    input_len: usize,
}

impl<S, I> TurboDecoding<S, I>
where
    S: BcjrState,
    I: Interleaver,
{
    pub fn get_result(&self) -> &[Llr] {
        &self.bcjr_result[0..self.input_len]
    }

    /// Run a single decode iteration
    pub fn run_decode_iteration(&mut self) {
        self.run_first_decoder();
        self.run_second_decoder();
    }

    fn run_first_decoder(&mut self) {
        self.first_bcjr
            .decode(&self.first_input, &mut self.bcjr_result);

        // Compute the extrinsic information from the a-posteriori LLR (Lapp) from the first decoder,
        // to be used as the a priori LLR for the second decoder.
        // This is eqn. 28. in Abrantes.

        for InterleaverMapping(i, ii) in self.interleaver {
            let lapp = self.bcjr_result[ii] as isize; // a-posteriori llr
            let la = self.first_input[ii].apriori as isize; // a-priori llr
            let lu = self.first_input[ii].systematic as isize;

            // Emit the extrinsic L_e from the first decoder as L_a for the second.
            let extrinsic = lapp - la - lu;

            self.second_input[i].apriori =
                extrinsic.clamp(Llr::MIN as isize, Llr::MAX as isize) as Llr;
        }
    }

    fn run_second_decoder(&mut self) {
        self.second_bcjr
            .decode(&self.second_input, &mut self.bcjr_result);

        // Compute the extrinsic information from the a-posteriori LLR (Lapp) from second decoder,
        // to be used now as the a-priori LLR for the first decoder.
        // This is eqn. 28. in Abrantes.

        for InterleaverMapping(i, ii) in self.interleaver {
            let lapp = self.bcjr_result[i] as isize; // a-posteriori llr
            let la = self.second_input[i].apriori as isize; // a-priori llr
            let lu = self.second_input[i].systematic as isize;

            // Emit the extrinsic L_e from the first decoder as L_a for the second.
            let extrinsic = lapp - la - lu;
            self.first_input[ii].apriori =
                extrinsic.clamp(Llr::MIN as isize, Llr::MAX as isize) as Llr;
        }

        // Deinterleave Lapp for decision making
        self.interleaver
            .deinterleave(&mut self.bcjr_result[0..self.input_len]);
    }
}

#[cfg(test)]
pub mod tests {
    use crate::interleaver::qpp::QppInterleaver;

    use super::*;

    #[test]
    fn can_decode_excel_example() {
        // Given
        let decoder = UmtsTurboDecoder::new(crate::catalog::UMTS);
        let interleaver = QppInterleaver::new(16, 1, 4);
        let mut iteration_results = Vec::new();

        let input = [
            TurboSymbol::new(-4, -4, -4),
            TurboSymbol::new(-4, -4, -4),
            TurboSymbol::new(-4, -4, -4),
            TurboSymbol::new(4, 4, 4),
            TurboSymbol::new(-4, 4, 4),
            TurboSymbol::new(-4, 4, 4),
            TurboSymbol::new(4, -4, -4),
            TurboSymbol::new(4, -4, 4),
            TurboSymbol::new(-4, -4, 4),
            TurboSymbol::new(-4, 4, -4),
            TurboSymbol::new(-4, 4, -4),
            TurboSymbol::new(-4, 4, 4),
            TurboSymbol::new(-4, -4, -4),
            TurboSymbol::new(-4, -4, 4),
            TurboSymbol::new(4, -4, -4),
            TurboSymbol::new(-4, 4, 4),
        ];
        let first_termination = [
            BcjrSymbol::new(4, 4),
            BcjrSymbol::new(-4, 4),
            BcjrSymbol::new(4, 4),
        ];
        let second_termination = [
            BcjrSymbol::new(-4, -4),
            BcjrSymbol::new(-4, -4),
            BcjrSymbol::new(-4, -4),
        ];

        // When
        let mut decoding =
            decoder.decode(&input, interleaver, &first_termination, &second_termination);

        iteration_results.push(decoding.get_result().to_vec());

        for _ in 0..2 {
            decoding.run_decode_iteration();
            iteration_results.push(decoding.get_result().to_vec());
        }

        // Then
        assert_eq!([0; 16].to_vec(), iteration_results[0]);
        assert_eq!(
            [-72, -52, -68, 44, -68, -72, 68, 68, -60, -72, -52, -60, -60, -52, 44, -52].to_vec(),
            iteration_results[1]
        );
        assert_eq!(
            [-108, -84, -92, 59, -92, -108, 88, 46, -76, -84, -60, -68, -76, -60, 44, -52].to_vec(),
            iteration_results[2]
        );
    }
}
