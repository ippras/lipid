use super::TriacylglycerolExpr;
use crate::polars::expr::ExprExt as _;
use atom::isotopes::*;
use polars::prelude::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;

impl TriacylglycerolExpr {
    /// C3H5 + \[RCOO\]-_{SN1} + \[RCOO\]-_{SN2} + \[RCOO\]-_{SN3} + ADDUCT
    pub fn mass(self, adduct: Option<Expr>) -> Expr {
        lit(3) * lit(C)
            + lit(5) * lit(H)
            + self
                .clone()
                .stereospecific_number1()
                .fatty_acid()
                .rcoo()
                .mass(None)
            + self
                .clone()
                .stereospecific_number2()
                .fatty_acid()
                .rcoo()
                .mass(None)
            + self.stereospecific_number3().fatty_acid().rcoo().mass(None)
            + adduct.unwrap_or(lit(0))
    }
}
