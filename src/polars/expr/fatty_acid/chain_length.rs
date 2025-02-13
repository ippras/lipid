use super::FattyAcidExpr;
use crate::polars::expr::chain_length::{
    EquivalentCarbonNumber, EquivalentChainLengths, FractionalChainLength, Options,
};
use polars::prelude::*;

impl EquivalentCarbonNumber for FattyAcidExpr {
    fn equivalent_carbon_number(self) -> Expr {
        self.clone().carbons() - lit(2) * self.unsaturated().sum()
    }
}

impl EquivalentChainLengths for FattyAcidExpr {
    fn equivalent_chain_lengths(self, retention_time: Expr, options: Options) -> Expr {
        self.clone()
            .saturated_or_null(self.clone().carbons())
            .forward_fill(None)
            + self.fcl(retention_time.clone(), options)
    }
}

impl FractionalChainLength for FattyAcidExpr {
    fn fractional_chain_length(self, retention_time: Expr, options: Options) -> Expr {
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
