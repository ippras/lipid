use super::TriacylglycerolExpr;
use crate::fatty_acid::{
    Kind,
    polars::{ExprExt as _, expr::mass::Mass as _},
};
use atom::isotopes::*;
use polars::prelude::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;

/// Mass
pub trait Mass {
    /// Mass
    fn mass(self, adduct: Expr) -> Expr;
}

impl Mass for TriacylglycerolExpr {
    /// C3H5 + SN1 [RCOO]- + SN2 [RCOO]- + SN3 [RCOO]- + ADDUCT
    fn mass(self, adduct: Expr) -> Expr {
        lit(3) * lit(C)
            + lit(5) * lit(H)
            + self.clone().sn1().fa().mass(Kind::Rcoo)
            + self.clone().sn2().fa().mass(Kind::Rcoo)
            + self.sn3().fa().mass(Kind::Rcoo)
            + adduct
    }
}
