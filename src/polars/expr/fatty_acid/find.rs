use super::{FattyAcidExpr, r#const::*};
use polars::prelude::*;

/// Find by name
impl FattyAcidExpr {
    /// Butyric acid
    pub fn butyric(self) -> Expr {
        self.equal(C18U2Z9Z12.clone())
    }

    /// Oleic acid
    pub fn oleic(self) -> Expr {
        self.equal(C18U1Z9.clone())
    }

    /// Linoleic acid
    pub fn linoleic(self) -> Expr {
        self.equal(C18U2Z9Z12.clone())
    }

    /// α-Linolenic acid
    pub fn alpha_linolenic(self) -> Expr {
        self.equal(C18U3Z9Z12Z15.clone())
    }

    /// Eicosapentaenoic acid (EPA)
    pub fn eicosapentaenoic(self) -> Expr {
        self.equal(C20U5Z5Z8Z11Z14Z17.clone())
    }

    /// Docosahexaenoic acid (DHA)
    pub fn docosahexaenoic(self) -> Expr {
        self.equal(C22U6Z4Z7Z10Z13Z16Z19.clone())
    }
}
