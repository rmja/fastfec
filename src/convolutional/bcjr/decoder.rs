use crate::{
    convolutional::{ConvolutionalCode, ConvolutionalCodeExt},
    Llr,
};
use core::{fmt::Debug, marker::PhantomData};
use heapless::Vec;

use super::BcjrSymbol;

pub struct BcjrDecoder<C: ConvolutionalCode, S: BcjrState, const MAX_TRELLIS_BITS: usize> {
    _code: PhantomData<C>,
    _state: PhantomData<S>,
    pub terminated: bool,
}

impl<C, S, const MAX_TRELLIS_BITS: usize> BcjrDecoder<C, S, MAX_TRELLIS_BITS>
where
    C: ConvolutionalCode,
    S: BcjrState,
{
    pub const fn new(terminated: bool) -> Self {
        Self {
            _code: PhantomData,
            _state: PhantomData,
            terminated,
        }
    }

    /// Soft decode a block of symbols
    pub fn decode(&self, input: &[BcjrSymbol], output: &mut [Llr]) {
        assert!(output.len() >= input.len());
        assert!(
            input.len() >= (1 + self.terminated as usize) * C::mem(),
            "The input is not long enough to open and possibly close the trellis"
        );

        let gamma = self.compute_gamma(input);
        let alpha = self.forward_path(&gamma);
        self.backward_path(gamma, alpha, output);
    }

    /// Compute the inner product of possible transmitted symbols and their received value.
    fn compute_gamma(&self, input: &[BcjrSymbol]) -> Vec<u32, MAX_TRELLIS_BITS> {
        let mut gamma = Vec::new();

        for symbol in input {
            let BcjrSymbol {
                systematic: lu,
                parity: lv,
                apriori: la,
            } = *symbol;
            // G from state emitting u=0/v=0: 0*La + 0*LU - 0*LV
            // G from state emitting u=0/v=1: 0*La + 0*LU + 1*LV
            // G from state emitting u=1/v=0: 1*La + 1*LU - 0*LV
            // G from state emitting u=1/v=1: 1*La + 1*LU + 1*LV

            let g0p1: i32 = lv as i32;
            let g1p0 = la as i32 + lu as i32;
            let g1p1 = g0p1 + g1p0;

            #[allow(clippy::identity_op)]
            let g = (0 << 0)
                | ((g0p1.clamp(i8::MIN as i32, i8::MAX as i32) as u8 as u32) << 8)
                | ((g1p0.clamp(i8::MIN as i32, i8::MAX as i32) as u8 as u32) << 16)
                | ((g1p1.clamp(i8::MIN as i32, i8::MAX as i32) as u8 as u32) << 24);
            gamma.push(g).unwrap();
        }

        gamma
    }

    fn forward_path(&self, gamma: &[u32]) -> Vec<S, MAX_TRELLIS_BITS> {
        let mut alpha = Vec::new();

        let symbol_count = gamma.len();
        let mut index = 0;

        let mut a = S::default();
        a = a.get_valid_scaled(index, symbol_count);
        alpha.push(a).unwrap();
        index += 1;

        while index < C::mem() {
            let g = gamma[index - 1];
            a = a.get_next_alpha(g);
            a = a.get_valid_scaled(index, symbol_count);
            alpha.push(a).unwrap();
            index += 1;
        }

        if self.terminated {
            // Trellis is terminated
            while index < symbol_count - C::mem() {
                let g = gamma[index - 1];
                a = a.get_next_alpha(g);
                a = a.get_all_scaled();
                alpha.push(a).unwrap();
                index += 1;
            }

            while index < symbol_count {
                let g = gamma[index - 1];
                a = a.get_next_alpha(g);
                a = a.get_valid_scaled(index, symbol_count);
                alpha.push(a).unwrap();
                index += 1;
            }
        } else {
            // Trellis is not terminated
            while index < symbol_count {
                let g = gamma[index - 1];
                a = a.get_next_alpha(g);
                a = a.get_all_scaled();
                alpha.push(a).unwrap();
                index += 1;
            }
        }

        assert_eq!(symbol_count, alpha.len());
        assert_eq!(symbol_count, index);

        alpha
    }

    fn backward_path(
        &self,
        gamma: Vec<u32, MAX_TRELLIS_BITS>,
        alpha: Vec<S, MAX_TRELLIS_BITS>,
        lapp: &mut [Llr],
    ) {
        let symbol_count = gamma.len();
        let mut index = symbol_count;

        let mut b = S::default();

        if self.terminated {
            b = b.get_valid_scaled(index, symbol_count);
            index -= 1;

            while index >= symbol_count - C::mem() {
                let g = gamma[index];
                let a = alpha[index];

                // Emit LLR
                lapp[index] = S::get_aposteriori(g, a, b);

                b = b.get_previous_beta(g);
                b = b.get_valid_scaled(index, symbol_count);
                index -= 1;
            }
        } else {
            index -= 1;
        }

        while index >= C::mem() {
            let g = gamma[index];
            let a = alpha[index];

            // Emit LLR
            lapp[index] = S::get_aposteriori(g, a, b);

            b = b.get_previous_beta(g);
            b = b.get_all_scaled();
            index -= 1;
        }

        while index > 0 {
            let g = gamma[index];
            let a = alpha[index];

            // Emit LLR
            lapp[index] = S::get_aposteriori(g, a, b);

            b = b.get_previous_beta(g);
            b = b.get_valid_scaled(index, symbol_count);
            index -= 1;
        }

        // index == 0
        {
            let g = gamma[index];
            let a = alpha[index];

            // Emit LLR
            lapp[index] = S::get_aposteriori(g, a, b);
        }
    }
}

pub trait BcjrState: Debug + Default + Copy {
    /// Get the unscaled next value of A in the forward path given the current value and `g`.
    fn get_next_alpha(self, g: u32) -> Self;

    /// Get the unscaled next value of B in the backward path given the current value and `g`.
    fn get_previous_beta(self, g: u32) -> Self;

    /// Get the a-posteriori llr given the three computation values.
    fn get_aposteriori(g: u32, a: Self, b: Self) -> Llr;

    /// Scale the values so that their sum is zero assuming all states are valid.
    fn get_all_scaled(self) -> Self;

    /// Invalidate unreachable states and scale remaining values so their sum is zero.
    fn get_valid_scaled(self, index: usize, symbol_count: usize) -> Self;
}
