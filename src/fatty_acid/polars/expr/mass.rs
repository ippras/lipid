use super::FattyAcidExpr;
use crate::fatty_acid::Kind;
use atom::isotopes::*;
use polars::prelude::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;
const O: f64 = O::Sixteen.relative_atomic_mass().value;

/// Mass
pub trait Mass {
    /// Mass
    fn mass(self, kind: Kind) -> Expr;
}

impl Mass for FattyAcidExpr {
    fn mass(self, kind: Kind) -> Expr {
        let mut c = self.clone().carbons();
        let mut h = self.hydrogens();
        let mut o = lit(2);
        match kind {
            Kind::Rcooh => {}
            Kind::Rcooch3 => {
                c = c + lit(1);
                h = h + lit(2);
            }
            Kind::Rcoo => h = h - lit(1),
            Kind::Rco => {
                h = h - lit(1);
                o = o - lit(1);
            }
        }
        c * lit(C) + h * lit(H) + o * lit(O)
    }
}
