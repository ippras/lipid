use polars::prelude::*;

/// Equivalent carbon number (ECN)
///
/// `ECN = C - 2U`
pub trait EquivalentCarbonNumber: Sized {
    fn equivalent_carbon_number(self) -> Expr;

    fn ecn(self) -> Expr {
        self.equivalent_carbon_number()
    }
}

/// Equivalent chain lengths (ECL)
pub trait EquivalentChainLengths: Sized {
    fn equivalent_chain_lengths(self, retention_time: Expr, options: Options) -> Expr;

    fn ecl(self, retention_time: Expr, options: Options) -> Expr {
        self.equivalent_chain_lengths(retention_time, options)
    }
}

/// Fractional chain length (FCL)
pub trait FractionalChainLength: Sized {
    fn fractional_chain_length(self, retention_time: Expr, options: Options) -> Expr;

    fn fcl(self, retention_time: Expr, options: Options) -> Expr {
        self.fractional_chain_length(retention_time, options)
    }
}

/// Chain length options
#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub logarithmic: bool,
}

impl Options {
    pub const fn new() -> Self {
        Self { logarithmic: false }
    }

    pub fn logarithmic(self, logarithmic: bool) -> Self {
        Self { logarithmic }
    }
}
