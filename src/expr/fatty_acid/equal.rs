use crate::prelude::*;
use polars::prelude::*;

impl FattyAcidExpr {
    /// Equal
    #[inline]
    pub fn equal(self, other: impl Into<FattyAcidExpr>) -> Expr {
        self.0.eq(other.into().0)
    }
}
