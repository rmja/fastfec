use crate::Llr;

#[derive(Clone, Copy, Debug)]
pub struct BcjrSymbol {
    pub systematic: Llr,
    pub parity: Llr,
    pub apriori: Llr,
}

impl BcjrSymbol {
    pub const fn new(systematic: Llr, parity: Llr) -> Self {
        Self {
            systematic,
            parity,
            apriori: 0,
        }
    }
}
