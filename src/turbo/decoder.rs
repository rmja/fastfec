use core::marker::PhantomData;

use crate::{
    convolutional::{
        bcjr::{umts::UmtsState, BcjrDecoder, BcjrState, BcjrSymbol},
        ConvolutionalCode,
    },
    interleaver::{Interleaver, InterleaverMapping},
    Llr,
};
use heapless::Vec;

use super::{code::assert_consituent_encoder, TurboCode, TurboSymbol};

pub struct TurboDecoder<C: TurboCode, S: BcjrState, const MAX_BLOCK_BITS: usize> {
    _code: PhantomData<C>,
    _state: PhantomData<S>,
}

pub type UmtsTurboDecoder<C, const MAX_BLOCK_BITS: usize> =
    TurboDecoder<C, UmtsState, MAX_BLOCK_BITS>;

impl<C: TurboCode, S: BcjrState, const MAX_BLOCK_BITS: usize> TurboDecoder<C, S, MAX_BLOCK_BITS> {
    const MAX_FIRST_TRELLIS_BITS: usize =
        trellis_bits::<C::ConstituentEncoderCode>(MAX_BLOCK_BITS, C::TERMINATE_FIRST);
    const MAX_SECOND_TRELLIS_BITS: usize =
        trellis_bits::<C::ConstituentEncoderCode>(MAX_BLOCK_BITS, C::TERMINATE_SECOND);
    const MAX_TRELLIS_BITS: usize = trellis_bits::<C::ConstituentEncoderCode>(
        MAX_BLOCK_BITS,
        C::TERMINATE_FIRST || C::TERMINATE_SECOND,
    );

    pub fn new() -> Self {
        assert_consituent_encoder::<C>();
        Self {
            _code: PhantomData,
            _state: PhantomData,
        }
    }

    pub fn decode<'a, I: Interleaver>(
        &self,
        input: &[TurboSymbol],
        interleaver: &'a I,
        first_termination: &[BcjrSymbol],
        second_termination: &[BcjrSymbol],
    ) -> TurboDecoding<
        'a,
        C,
        S,
        I,
        MAX_BLOCK_BITS,
        { Self::MAX_FIRST_TRELLIS_BITS },
        { Self::MAX_SECOND_TRELLIS_BITS },
        { Self::MAX_TRELLIS_BITS },
    > {
        // Prepare input for the first decoder
        let mut first_input = Vec::new();
        for symbol in input {
            first_input
                .push(BcjrSymbol::new(symbol.systematic, symbol.first_parity))
                .unwrap()
        }
        first_input.extend_from_slice(first_termination).unwrap();

        // Prepare input for the second decoder
        let mut second_input = Vec::new();
        for InterleaverMapping(i, ii) in interleaver.iter() {
            second_input
                .push(BcjrSymbol::new(
                    input[ii].systematic,
                    input[i].second_parity,
                ))
                .unwrap();
        }
        second_input.extend_from_slice(second_termination).unwrap();

        // Create a result buffer that the individual decoders can use
        let mut bcjr_result = Vec::new();
        bcjr_result
            .resize_default(
                input.len() + usize::max(first_termination.len(), second_termination.len()),
            )
            .unwrap();

        TurboDecoding {
            _code: PhantomData,
            first_bcjr: BcjrDecoder::new(C::TERMINATE_FIRST),
            second_bcjr: BcjrDecoder::new(C::TERMINATE_SECOND),
            first_input,
            second_input,
            interleaver,
            bcjr_result,
            input_len: input.len(),
        }
    }
}

pub const fn trellis_bits<C: ConvolutionalCode>(block_bits: usize, terminated: bool) -> usize {
    block_bits
        + if terminated {
            C::CONSTRAINT_LENGTH - 1
        } else {
            0
        }
}

impl<C: TurboCode, S: BcjrState, const MAX_BLOCK_BITS: usize> Default
    for TurboDecoder<C, S, MAX_BLOCK_BITS>
{
    fn default() -> Self {
        TurboDecoder::new()
    }
}

pub struct TurboDecoding<
    'a,
    C: TurboCode,
    S: BcjrState,
    I: Interleaver,
    const MAX_BLOCK_BITS: usize,
    const MAX_FIRST_TRELLIS_BITS: usize,
    const MAX_SECOND_TRELLIS_BITS: usize,
    const MAX_TRELLIS_BITS: usize,
> {
    _code: PhantomData<C>,
    first_bcjr: BcjrDecoder<C::ConstituentEncoderCode, S, MAX_TRELLIS_BITS>,
    second_bcjr: BcjrDecoder<C::ConstituentEncoderCode, S, MAX_TRELLIS_BITS>,
    interleaver: &'a I,
    first_input: Vec<BcjrSymbol, MAX_FIRST_TRELLIS_BITS>,
    second_input: Vec<BcjrSymbol, MAX_SECOND_TRELLIS_BITS>,
    bcjr_result: Vec<Llr, MAX_TRELLIS_BITS>,
    input_len: usize,
}

impl<
        C,
        S,
        I,
        const MAX_BLOCK_BITS: usize,
        const MAX_FIRST_TRELLIS_BITS: usize,
        const MAX_SECOND_TRELLIS_BITS: usize,
        const MAX_TRELLIS_BITS: usize,
    >
    TurboDecoding<
        '_,
        C,
        S,
        I,
        MAX_BLOCK_BITS,
        MAX_FIRST_TRELLIS_BITS,
        MAX_SECOND_TRELLIS_BITS,
        MAX_TRELLIS_BITS,
    >
where
    C: TurboCode,
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

        for InterleaverMapping(i, ii) in self.interleaver.iter() {
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

        for InterleaverMapping(i, ii) in self.interleaver.iter() {
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
mod tests {
    use crate::{catalog, interleaver::qpp::QppInterleaver};

    use super::*;

    #[test]
    fn can_decode_excel_example() {
        // Given
        let decoder = UmtsTurboDecoder::<catalog::UMTS, 16>::default();
        let interleaver = QppInterleaver::new(16, 1, 4);
        let mut iteration_results = Vec::<_, 3>::new();

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
        let mut decoding = decoder.decode(
            &input,
            &interleaver,
            &first_termination,
            &second_termination,
        );

        iteration_results
            .push(decoding.get_result().to_vec())
            .unwrap();

        for _ in 0..2 {
            decoding.run_decode_iteration();
            iteration_results
                .push(decoding.get_result().to_vec())
                .unwrap();
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
