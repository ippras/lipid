use crate::{polars::expr::FattyAcidExpr, prelude::*};
use atom::isotopes::*;
use polars::prelude::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;
const O: f64 = O::Sixteen.relative_atomic_mass().value;

impl FattyAcidExpr {
    /// Calculates the mass of a fatty acid expression.
    ///
    /// # Arguments
    ///
    /// * `adduct` - An optional expression representing an adduct mass.
    ///
    /// # Returns
    ///
    /// An expression representing the calculated mass.
    pub fn mass(self, adduct: Option<Expr>) -> Expr {
        self.clone().carbons() * lit(C)
            + self.hydrogens() * lit(H)
            + lit(2) * lit(O)
            + adduct.unwrap_or(lit(0))
    }
}

impl Rco<FattyAcidExpr> {
    /// Calculates the mass of an RCO fatty acid expression.
    ///
    /// # Arguments
    ///
    /// * `adduct` - An optional expression representing an adduct mass.
    ///
    /// # Returns
    ///
    /// An expression representing the calculated mass.
    pub fn mass(self, adduct: Option<Expr>) -> Expr {
        self.0.mass(adduct) - lit(H) - lit(O)
    }
}

impl Rcoo<FattyAcidExpr> {
    /// Calculates the mass of an RCOO fatty acid expression.
    ///
    /// # Arguments
    ///
    /// * `adduct` - An optional expression representing an adduct mass.
    ///
    /// # Returns
    ///
    /// An expression representing the calculated mass.
    pub fn mass(self, adduct: Option<Expr>) -> Expr {
        self.0.mass(adduct) - lit(H)
    }
}

impl Rcooh<FattyAcidExpr> {
    /// Calculates the mass of an RCOOH fatty acid expression.
    ///
    /// # Arguments
    ///
    /// * `adduct` - An optional expression representing an adduct mass.
    ///
    /// # Returns
    ///
    /// An expression representing the calculated mass.
    pub fn mass(self, adduct: Option<Expr>) -> Expr {
        self.0.mass(adduct)
    }
}

impl Rcooch3<FattyAcidExpr> {
    /// Calculates the mass of an RCOOCH3 fatty acid expression.
    ///
    /// # Arguments
    ///
    /// * `adduct` - An optional expression representing an adduct mass.
    ///
    /// # Returns
    ///
    /// An expression representing the calculated mass.
    pub fn mass(self, adduct: Option<Expr>) -> Expr {
        self.0.mass(adduct) + lit(2) * lit(H) + lit(C)
    }
}
