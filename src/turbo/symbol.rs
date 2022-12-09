use crate::Llr;

pub struct TurboSymbol {
    pub systematic: Llr,
    pub first_parity: Llr,
    pub second_parity: Llr,
}

impl TurboSymbol {
    pub const fn new(systematic: Llr, first_parity: Llr, second_parity: Llr) -> Self {
        Self {
            systematic,
            first_parity,
            second_parity,
        }
    }
}
