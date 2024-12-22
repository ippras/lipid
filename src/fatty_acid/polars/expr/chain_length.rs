use super::FattyAcidExpr;
use polars::prelude::*;

/// Chain length methods for [`FattyAcid`]
pub trait ChainLength {
    /// Equivalent chain lengths (ECL)
    fn ecl(self, retention_time: Expr, options: Options) -> Expr;

    /// Fractional chain length (FCL)
    fn fcl(self, retention_time: Expr, options: Options) -> Expr;
}

impl ChainLength for FattyAcidExpr {
    fn ecl(self, retention_time: Expr, options: Options) -> Expr {
        self.clone()
            .saturated_or_null(self.clone().carbons())
            .forward_fill(None)
            + self.fcl(retention_time.clone(), options)
    }

    fn fcl(self, retention_time: Expr, options: Options) -> Expr {
        let options = |mut expr: Expr| {
            if options.logarithmic {
                expr = expr.log(10.0)
            }
            expr
        };
        let unsaturated_time = || options(retention_time.clone());
        let saturated_time = || options(self.clone().saturated_or_null(retention_time.clone()));
        let saturated_carbons = || self.clone().saturated_or_null(self.clone().carbons());
        ternary_expr(
            self.clone().is_saturated(),
            lit(0),
            (saturated_carbons().backward_fill(None) - saturated_carbons().forward_fill(None))
                * (unsaturated_time() - saturated_time().forward_fill(None))
                / (saturated_time().backward_fill(None) - saturated_time().forward_fill(None)),
        )
    }
}

/// FCL options
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
