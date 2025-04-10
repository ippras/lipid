use super::TriacylglycerolExpr;
use crate::polars::ExprExt as _;
use polars::prelude::*;

impl TriacylglycerolExpr {
    pub fn equivalent_carbon_number(self) -> Expr {
        (self
            .clone()
            .stereospecific_number1()
            .fatty_acid()
            .equivalent_carbon_number()
            + self
                .clone()
                .stereospecific_number2()
                .fatty_acid()
                .equivalent_carbon_number()
            + self
                .stereospecific_number3()
                .fatty_acid()
                .equivalent_carbon_number())
        .alias("EquivalentCarbonNumber")
    }
}
