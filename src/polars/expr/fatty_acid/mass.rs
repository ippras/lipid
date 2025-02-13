use super::{Rco, Rcoo, Rcooch3, Rcooh, kind::FattyAcidExprExt as _};
use crate::polars::expr::{FattyAcidExpr, mass::Mass};
use atom::isotopes::*;
use polars::prelude::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;
const O: f64 = O::Sixteen.relative_atomic_mass().value;

impl Mass for FattyAcidExpr {
    fn mass(self, adduct: Option<Expr>) -> Expr {
        self.rcooh().mass(adduct)
    }
}

impl Mass for Rco {
    fn mass(self, adduct: Option<Expr>) -> Expr {
        self.0.mass(adduct) - lit(H) - lit(O)
    }
}

impl Mass for Rcoo {
    fn mass(self, adduct: Option<Expr>) -> Expr {
        self.0.mass(adduct) - lit(H)
    }
}

impl Mass for Rcooh {
    fn mass(self, adduct: Option<Expr>) -> Expr {
        let c = self.0.clone().carbons();
        let h = self.0.hydrogens();
        let o = lit(2);
        c * lit(C) + h * lit(H) + o * lit(O) + adduct.unwrap_or(lit(0))
    }
}

impl Mass for Rcooch3 {
    fn mass(self, adduct: Option<Expr>) -> Expr {
        self.0.mass(adduct) + lit(2) * lit(H) + lit(C)
    }
}
