use crate::{polars::expr::FattyAcidExpr, prelude::*};
use atom::isotopes::*;
use polars::prelude::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;
const O: f64 = O::Sixteen.relative_atomic_mass().value;

impl Mass for FattyAcidExpr {
    type Output = Expr;

    fn mass(self, adduct: Option<Expr>) -> Expr {
        (self.clone().carbons() * lit(C)
            + self.hydrogens() * lit(H)
            + lit(2) * lit(O)
            + adduct.unwrap_or(lit(0)))
        .alias("Mass")
    }
}

impl Mass for Rco<FattyAcidExpr> {
    type Output = Expr;

    fn mass(self, adduct: Option<Expr>) -> Expr {
        self.0.mass(adduct) - lit(H) - lit(O)
    }
}

impl Mass for Rcoo<FattyAcidExpr> {
    type Output = Expr;

    fn mass(self, adduct: Option<Expr>) -> Expr {
        self.0.mass(adduct) - lit(H)
    }
}

impl Mass for Rcooh<FattyAcidExpr> {
    type Output = Expr;

    fn mass(self, adduct: Option<Expr>) -> Expr {
        self.0.mass(adduct)
    }
}

impl Mass for Rcooch3<FattyAcidExpr> {
    type Output = Expr;

    fn mass(self, adduct: Option<Expr>) -> Expr {
        self.0.mass(adduct) + lit(2) * lit(H) + lit(C)
    }
}
