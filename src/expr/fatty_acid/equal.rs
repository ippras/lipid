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
        self.equal(C18DC9DC12.clone())
    }

    /// Oleic acid
    pub fn oleic(self) -> Expr {
        self.equal(C18DC9.clone())
    }

    /// Linoleic acid
    pub fn linoleic(self) -> Expr {
        self.equal(C18DC9DC12.clone())
    }

    /// α-Linolenic acid
    pub fn alpha_linolenic(self) -> Expr {
        self.equal(C18DC9DC12DC15.clone())
    }

    /// Eicosapentaenoic acid (EPA)
    pub fn eicosapentaenoic(self) -> Expr {
        self.equal(C20DC5DC8DC11DC14DC17.clone())
    }

    /// Docosahexaenoic acid (DHA)
    pub fn docosahexaenoic(self) -> Expr {
        self.equal(C22DC4DC7DC10DC13DC16DC19.clone())
    }
}
