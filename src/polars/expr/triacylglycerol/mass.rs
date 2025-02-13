use super::TriacylglycerolExpr;
use crate::polars::expr::{ExprExt as _, fatty_acid::kind::FattyAcidExprExt as _, mass::Mass};
use atom::isotopes::*;
use polars::prelude::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;

impl Mass for TriacylglycerolExpr {
    /// C3H5 + [RCOO]-_{SN1} + [RCOO]-_{SN2} + [RCOO]-_{SN3} + ADDUCT
    fn mass(self, adduct: Option<Expr>) -> Expr {
        lit(3) * lit(C)
            + lit(5) * lit(H)
            + self.clone().sn1().fa().rcoo().mass(None)
            + self.clone().sn2().fa().rcoo().mass(None)
            + self.sn3().fa().rcoo().mass(None)
            + adduct.unwrap_or(lit(0))
    }
}
