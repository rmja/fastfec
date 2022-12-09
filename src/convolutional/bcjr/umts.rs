use core::simd::{i8x8, SimdOrd, SimdInt};

use crate::Llr;

use super::{decoder::BcjrState, BcjrDecoder};

pub type UmtsBcjrDecoder = BcjrDecoder<UmtsState>;

#[derive(Clone, Copy)]
pub union UmtsState {
    value: Value,
    simd: i8x8,
    #[allow(dead_code)]
    debug: [i8; 8],
}

#[derive(Clone, Copy)]
struct Value {
    s74: u32,
    s30: u32,
}

impl UmtsState {
    const fn new(s74: u32, s30: u32) -> Self {
        Self {
            value: Value { s74, s30 },
        }
    }

    const fn split(self) -> Value {
        unsafe { self.value }
    }

    const fn simd(self) -> i8x8 {
        unsafe { self.simd }
    }

    fn simd_saturating_add(left: Self, right: Self) -> Self {
        Self {
            simd: left.simd().saturating_add(right.simd()),
        }
    }

    fn simd_saturating_sub(left: Self, right: Self) -> Self {
        Self {
            simd: left.simd().saturating_sub(right.simd()),
        }
    }

    fn simd_max(x: Self, y: Self) -> Self {
        Self {
            simd: x.simd().simd_max(y.simd()),
        }
    }

    fn reduce_max(self) -> i8 {
        self.simd().reduce_max()
    }

    fn reduce_sum(self) -> i16 {
        self.simd().cast::<i16>().reduce_sum()
    }

    const fn count_ones(self) -> u32 {
        let Value { s74, s30 } = self.split();
        s74.count_ones() + s30.count_ones()
    }

    const fn get_mask(index: usize, symbol_count: usize) -> Self {
        match index {
            0 => Self::new(0x00000000, 0x000000FF), // S0 is valid
            1 => Self::new(0x000000FF, 0x000000FF), // S0 and S4 are valid
            2 => Self::new(0x00FF00FF, 0x00FF00FF), // S0, S2, S4 and S6 are valid
            _ if index == symbol_count - 2 => Self::new(0x00000000, 0xFFFFFFFF), // S0, S1, S2 and S3 are valid
            _ if index == symbol_count - 1 => Self::new(0x00000000, 0x0000FFFF), // S0 and S1 are valid
            _ if index == symbol_count => Self::new(0x00000000, 0x000000FF),     // S0 is valid
            _ => Self::new(0xFFFFFFFF, 0xFFFFFFFF), // All states are valid
        }
    }

    /// Get the scale coefficients so that all values accross all states sum to 0 as log(1) = 0
    fn get_scale_coefficients(masked_unscaled: Self, valid_state_count: usize) -> Self {
        let sum = masked_unscaled.reduce_sum();
        let coeff = (sum / valid_state_count as i16) as i8;
        Self {
            simd: i8x8::splat(coeff),
        }
    }
}

impl Default for UmtsState {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl BcjrState for UmtsState {
    fn get_next_alpha(self, g: u32) -> Self {
        let Value { s74, s30 } = self.split();

        // Case when u=0 is transmitted
        let mut a0 = Self::new(
            // pr     cr u/v
            (s74 & 0x00FFFF00) << 8 |   // s6 --> s7 0/0
                                            // s5 --> s6 0/1
                (s30 & 0x00FFFF00) >> 8, // s2 --> s5 0/1
            // s1 --> s4 0/0
            (s74 & 0xFF000000) |          // s7 --> s3 0/0
                (s74 & 0x000000FF) << 16 |  // s4 --> s2 0/1
                (s30 & 0xFF000000) >> 16 |  // s3 --> s1 0/1
                (s30 & 0x000000FF), // s0 --> s0 0/0
        );

        let g0 = Self::new(
            // pr     cr u/v
            (g & 0x000000FF) << 24 |    // s6 <-> s7 0/0
                (g & 0x0000FF00) << 8 |     // s5 <-> s6 0/1
                (g & 0x0000FFFF), // s2 <-> s5 0/1
            // s1 <-> s4 0/0
            (g & 0x000000FF) << 24 |    // s7 <-> s3 0/0
                (g & 0x0000FF00) << 8 |     // s4 <-> s2 0/1
                (g & 0x0000FFFF), // s3 <-> s1 0/1
                                  // s0 <-> s0 0/0
        );

        a0 = Self::simd_saturating_add(a0, g0);

        // Case when u=1 is transmitted
        let mut a1 = Self::new(
            // pr     cr u/v
            (s74 & 0xFF000000) |          // s7 --> s7 1/1
                (s74 & 0x000000FF) << 16 |  // s4 --> s6 1/0
                (s30 & 0xFF000000) >> 16 |  // s3 --> s5 1/0
                (s30 & 0x000000FF), // s0 --> s4 1/1
            (s74 & 0x00FFFF00) << 8 |   // s6 --> s3 1/1
                                            // s5 --> s2 1/0
                (s30 & 0x00FFFF00) >> 8, // s2 --> s1 1/0
                                         // s1 --> s0 1/1
        );

        let g1 = Self::new(
            // pr     cr u/v
            (g & 0xFFFF0000) |            // s7 <-> s7 1/1
                                            // s4 <-> s6 1/0
                (g & 0x00FF0000) >> 8 |     // s3 <-> s5 1/0
                (g & 0xFF000000) >> 24, // s0 <-> s4 1/1
            (g & 0xFFFF0000) |            // s6 <-> s3 1/1
                                            // s5 <-> s2 1/0
                (g & 0x00FF0000) >> 8 |     // s2 <-> s1 1/0
                (g & 0xFF000000) >> 24, // s1 <-> s0 1/1
        );

        a1 = Self::simd_saturating_add(a1, g1);

        Self::simd_max(a0, a1)
    }

    fn get_previous_beta(self, g: u32) -> Self {
        let Value { s74, s30 } = self.split();

        // Case when u=0 is transmitted.
        let mut b0 = Self::new(
            // cr     nx u/v
            (s30 & 0xFF000000) |        // s7 <-- s3 0/0
                (s74 & 0xFFFF0000) >> 8 |   // s6 <-- s7 0/0
                                            // s5 <-- s6 0/1
                (s30 & 0x00FF0000) >> 16, // s4 <-- s2 0/1
            (s30 & 0x0000FF00) << 16 |  // s3 <-- s1 0/1
                (s74 & 0x0000FFFF) << 8 |   // s2 <-- s5 0/1
                                            // s1 <-- s4 0/0
                (s30 & 0x000000FF), // s0 <-- s0 0/0
        );

        let g0 = Self::new(
            // cr     nx u/v
            (g & 0x000000FF) << 24 |    // s7 <-> s3 0/0
                (g & 0x000000FF) << 16 |    // s6 <-> s7 0/0
                (g & 0x0000FF00) |          // s5 <-> s6 0/1
                (g & 0x0000FF00) >> 8, // s4 <-> s2 0/1
            (g & 0x0000FF00) << 16 |    // s3 <-> s1 0/1
                (g & 0x0000FFFF) << 8 |     // s2 <-> s5 0/1
                                            // s1 <-> s4 0/0
                (g & 0x000000FF), // s0 <-> s0 0/0
        );

        b0 = Self::simd_saturating_add(b0, g0);

        // Case when u=1 is transmitted
        let mut b1 = Self::new(
            // cr     nx u/v
            (s74 & 0xFF000000) |        // s7 <-- s7 1/1
                (s30 & 0xFFFF0000) >> 8 |   // s6 <-- s3 1/1
                                            // s5 <-- s2 1/0
                (s74 & 0x00FF0000) >> 16, // s4 <-- s6 1/0
            (s74 & 0x0000FF00) << 16 |  // s3 <-- s5 1/0
                (s30 & 0x0000FFFF) << 8 |   // s2 <-- s1 1/0
                                            // s1 <-- s0 1/1
                (s74 & 0x000000FF), // s0 <-- s4 1/1
        );

        let g1 = Self::new(
            // cr     nx u/v
            (g & 0xFF000000) |          // s7 <-> s7 1/1
                (g & 0xFFFF0000) >> 8 |     // s6 <-> s3 1/1
                                            // s5 <-> s2 1/0
                (g & 0x00FF0000) >> 16, // s4 <-> s6 1/0
            (g & 0x00FF0000) << 8 |     // s3 <-> s5 1/0
                (g & 0x00FF0000) |          // s2 <-> s1 1/0
                (g & 0xFF000000) >> 16 |    // s1 <-> s0 1/1
                (g & 0xFF000000) >> 24, // s0 <-> s4 1/1
        );

        b1 = Self::simd_saturating_add(b1, g1);

        Self::simd_max(b0, b1)
    }

    fn get_aposteriori(g: u32, a: Self, b: Self) -> Llr {
        let Value { s74: b74, s30: b30 } = b.split();

        // Align g for u=0
        let g0 = Self::new(
            (g & 0x000000FF) << 24 |    // s7 <-> s3 0/0
                (g & 0x000000FF) << 16 |    // s6 <-> s7 0/0
                (g & 0x0000FF00) |          // s5 <-> s6 0/1
                (g & 0x0000FF00) >> 8, // s4 <-> s2 0/1
            (g & 0x0000FF00) << 16 |    // s3 <-> s1 0/1
                (g & 0x0000FFFF) << 8 |     // s2 <-> s5 0/1
                                            // s1 <-> s4 0/0
                (g & 0x000000FF), // s0 <-> s0 0/0
        );

        // Align B for u=0 according to A
        let b0 = Self::new(
            (b30 & 0xFF000000) |      // s7 <-- s3 0/0
                (b74 & 0xFFFF0000) >> 8 | // s6 <-- s7 0/0
                                            // s5 <-- s6 0/1
                (b30 & 0x00FF0000) >> 16, // s4 <-- s2 0/1
            (b30 & 0x0000FF00) << 16 |// s3 <-- s1 0/1
                (b74 & 0x0000FFFF) << 8 | // s2 <-- s5 0/1
                                            // s1 <-- s4 0/0
                (b30 & 0x000000FF), // s0 <-- s0 0/0
        );

        let sum0 = Self::simd_saturating_add(Self::simd_saturating_add(a, g0), b0);
        let max0 = sum0.reduce_max();

        // Align g for u=1
        let g1 = Self::new(
            (g & 0xFF000000) |          // s7 <-> s7 1/1
                (g & 0xFFFF0000) >> 8 |     // s6 <-> s3 1/1
                                            // s5 <-> s2 1/0
                (g & 0x00FF0000) >> 16, // s4 <-> s6 1/0
            (g & 0x00FF0000) << 8 |     // s3 <-> s5 1/0
                (g & 0x00FF0000) |          // s2 <-> s1 1/0
                (g & 0xFF000000) >> 16 |    // s1 <-> s0 1/1
                (g & 0xFF000000) >> 24, // s0 <-> s4 1/1
        );

        // Align B for u=1 according to A.
        let b1 = Self::new(
            (b74 & 0xFF000000) |      // s7 <-- s7 1/1
                (b30 & 0xFFFF0000) >> 8 | // s6 <-- s3 1/1
                                            // s5 <-- s2 1/0
                (b74 & 0x00FF0000) >> 16, // s4 <-- s6 1/0
            (b74 & 0x0000FF00) << 16 |// s3 <-- s5 1/0
                (b30 & 0x0000FFFF) << 8 | // s2 <-- s1 1/0
                                            // s1 <-- s0 1/1
                (b74 & 0x000000FF), // s0 <-- s4 1/1
        );

        let sum1 = Self::simd_saturating_add(Self::simd_saturating_add(a, g1), b1);
        let max1 = sum1.reduce_max();

        (max1 as i32 - max0 as i32).clamp(i8::MIN as i32, i8::MAX as i32) as Llr
    }

    fn get_all_scaled(self) -> Self {
        let coefficients = Self::get_scale_coefficients(self, 8);
        let scaled = Self::simd_saturating_sub(self, coefficients);
        scaled
    }

    fn get_valid_scaled(self, index: usize, symbol_count: usize) -> Self {
        let s = self.split();
        let mask = Self::get_mask(index, symbol_count);
        let valid_state_count = mask.count_ones() as usize / 8;
        const UNREACHABLE: u32 = 0x80808080; // -Infinity for all states

        let mask = mask.split();
        let masked = Self::new(s.s74 & mask.s74, s.s30 & mask.s30);

        let coefficients = Self::get_scale_coefficients(masked, valid_state_count);
        let scaled = Self::simd_saturating_sub(masked, coefficients).split();
        let invalidated = Self::new(
            (scaled.s74 & mask.s74) | (UNREACHABLE & !mask.s74),
            (scaled.s30 & mask.s30) | (UNREACHABLE & !mask.s30),
        );
        invalidated
    }
}

#[cfg(test)]
mod tests {
    use crate::{catalog, convolutional::bcjr::BcjrSymbol};

    use super::*;

    #[test]
    fn can_decode_byte() {
        // Given
        let decoder = UmtsBcjrDecoder::new(catalog::UMTS_CONSTITUENT, true);
        let input = [
            BcjrSymbol::new(4, 4),
            BcjrSymbol::new(4, -4),
            BcjrSymbol::new(-4, -4),
            BcjrSymbol::new(4, 4),
            BcjrSymbol::new(4, 4),
            BcjrSymbol::new(-4, -4),
            BcjrSymbol::new(-4, 4),
            BcjrSymbol::new(4, 4),
            BcjrSymbol::new(-4, -4),
            BcjrSymbol::new(-4, -4),
            BcjrSymbol::new(-4, -4),
        ];
        let mut output = [0; 11];

        // When
        decoder.decode(&input, &mut output);

        // Then
        assert_eq!([24, 24, -24, 24, 24, -24, -24, 24, -24, -24, -24], output);
    }

    #[test]
    fn can_decode_excel_example_decoder1() {
        // Given
        let decoder = UmtsBcjrDecoder::new(catalog::UMTS_CONSTITUENT, true);
        let input = [
            BcjrSymbol::new(-4, -4),
            BcjrSymbol::new(-4, -4),
            BcjrSymbol::new(-4, -4),
            BcjrSymbol::new(4, 4),
            BcjrSymbol::new(-4, 4),
            BcjrSymbol::new(-4, 4),
            BcjrSymbol::new(4, -4),
            BcjrSymbol::new(4, -4),
            BcjrSymbol::new(-4, -4),
            BcjrSymbol::new(-4, 4),
            BcjrSymbol::new(-4, 4),
            BcjrSymbol::new(-4, 4),
            BcjrSymbol::new(-4, -4),
            BcjrSymbol::new(-4, -4),
            BcjrSymbol::new(4, -4),
            BcjrSymbol::new(-4, 4),
            BcjrSymbol::new(4, 4),
            BcjrSymbol::new(-4, 4),
            BcjrSymbol::new(4, 4),
        ];
        let mut output = [0; 19];

        // When
        decoder.decode(&input, &mut output);

        // Then
        assert_eq!(
            [
                -24, -24, -24, 24, -24, -24, 24, 24, -24, -24, -24, -24, -24, -24, 24, -24, 24,
                -24, 24
            ],
            output
        );
    }
}
