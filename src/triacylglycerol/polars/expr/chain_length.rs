use super::TriacylglycerolExpr;
use crate::{chain_length::EquivalentCarbonNumber, fatty_acid::polars::ExprExt as _};
use polars::prelude::*;

impl EquivalentCarbonNumber for TriacylglycerolExpr {
    fn equivalent_carbon_number(self) -> Expr {
        self.clone().sn1().fa().ecn() + self.clone().sn2().fa().ecn() + self.sn3().fa().ecn()
    }
}
