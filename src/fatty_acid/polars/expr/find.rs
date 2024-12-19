use super::{FattyAcidExpr, r#const::*};
use polars::prelude::*;

/// Find by name
pub trait FindByName {
    /// Butyric acid
    fn butyric(self) -> Expr;

    /// Oleic acid
    fn oleic(self) -> Expr;

    /// Linoleic acid
    fn linoleic(self) -> Expr;

    /// α-Linolenic acid
    fn alpha_linolenic(self) -> Expr;

    /// Eicosapentaenoic acid (EPA)
    fn eicosapentaenoic(self) -> Expr;

    /// Docosahexaenoic acid (DHA)
    fn docosahexaenoic(self) -> Expr;
}

impl FindByName for FattyAcidExpr {
    fn butyric(self) -> Expr {
        self.equal(C18U2Z9Z12.clone())
    }

    fn oleic(self) -> Expr {
        self.equal(C18U1Z9.clone())
    }

    fn linoleic(self) -> Expr {
        self.equal(C18U2Z9Z12.clone())
    }

    fn alpha_linolenic(self) -> Expr {
        self.equal(C18U3Z9Z12Z15.clone())
    }

    fn eicosapentaenoic(self) -> Expr {
        self.equal(C20U5Z5Z8Z11Z14Z17.clone())
    }

    fn docosahexaenoic(self) -> Expr {
        self.equal(C22U6Z4Z7Z10Z13Z16Z19.clone())
    }
}
