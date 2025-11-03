use crate::prelude::*;
use atom::isotopes::*;
use polars::prelude::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;
const O: f64 = O::Sixteen.relative_atomic_mass().value;

impl RelativeAtomicMass for FattyAcidExpr {
    type Output = Expr;

    fn relative_atomic_mass(self, adduct: Option<Expr>) -> Expr {
        (self.clone().carbon() * lit(C)
            + self.clone().hydrogen() * lit(H)
            + self.oxygen() * lit(O)
            + adduct.unwrap_or(lit(0)))
        .alias("Mass")
    }
}

impl RelativeAtomicMass for Rco<FattyAcidExpr> {
    type Output = Expr;

    fn relative_atomic_mass(self, adduct: Option<Expr>) -> Expr {
        self.0.relative_atomic_mass(adduct) - lit(H) - lit(O)
    }
}

impl RelativeAtomicMass for Rcoo<FattyAcidExpr> {
    type Output = Expr;

    fn relative_atomic_mass(self, adduct: Option<Expr>) -> Expr {
        self.0.relative_atomic_mass(adduct) - lit(H)
    }
}

impl RelativeAtomicMass for Rcooh<FattyAcidExpr> {
    type Output = Expr;

    fn relative_atomic_mass(self, adduct: Option<Expr>) -> Expr {
        self.0.relative_atomic_mass(adduct)
    }
}

impl RelativeAtomicMass for Rcooch3<FattyAcidExpr> {
    type Output = Expr;

    fn relative_atomic_mass(self, adduct: Option<Expr>) -> Expr {
        self.0.relative_atomic_mass(adduct) + lit(2) * lit(H) + lit(C)
    }
}
