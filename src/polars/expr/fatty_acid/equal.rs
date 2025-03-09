use crate::prelude::*;
use polars::prelude::*;

/// Find by name
impl FattyAcidExpr {
    /// Equal
    #[inline]
    pub fn equal(self, other: impl Into<FattyAcidExpr>) -> Expr {
        self.0.eq(other.into().0)
    }

    /// Butyric acid
    pub fn butyric(self) -> Expr {
        self.equal(C18U2DC9DC12)
    }

    /// Oleic acid
    pub fn oleic(self) -> Expr {
        self.equal(C18U1DC9)
    }

    /// Linoleic acid
    pub fn linoleic(self) -> Expr {
        self.equal(C18U2DC9DC12)
    }

    /// α-Linolenic acid
    pub fn alpha_linolenic(self) -> Expr {
        self.equal(C18U3DC9DC12DC15)
    }

    /// Eicosapentaenoic acid (EPA)
    pub fn eicosapentaenoic(self) -> Expr {
        self.equal(C20U5DC5DC8DC11DC14DC17)
    }

    /// Docosahexaenoic acid (DHA)
    pub fn docosahexaenoic(self) -> Expr {
        self.equal(C22U6DC4DC7DC10DC13DC16DC19)
    }
}
