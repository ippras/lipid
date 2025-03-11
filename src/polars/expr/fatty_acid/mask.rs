use super::FattyAcidExpr;
use crate::prelude::*;
use polars::prelude::*;
use std::num::NonZeroI8;

impl FattyAcidExpr {
    #[inline]
    pub fn mask(
        self,
        f: impl Fn(&FattyAcidChunked) -> PolarsResult<BooleanChunked> + Send + Sync + 'static,
    ) -> Expr {
        self.0.map(
            move |column| Ok(Some(f(column.fatty_acid()?)?.into_column())),
            GetOutput::from_type(DataType::Boolean),
        )
    }

    /// Is saturated
    #[inline]
    pub fn is_saturated(self) -> Expr {
        self.mask(|fatty_acids| fatty_acids.is_saturated())
    }

    /// Is unsaturated
    #[inline]
    pub fn is_unsaturated(self, n: Option<NonZeroI8>) -> Expr {
        self.mask(move |fatty_acids| fatty_acids.is_unsaturated(n))
    }

    /// Is monounsaturated
    #[inline]
    pub fn is_monounsaturated(self) -> Expr {
        self.mask(|fatty_acids| fatty_acids.is_monounsaturated())
    }

    /// Is polyunsaturated
    #[inline]
    pub fn is_polyunsaturated(self) -> Expr {
        self.mask(|fatty_acids| fatty_acids.is_polyunsaturated())
    }

    /// Is cis
    #[inline]
    pub fn is_cis(self) -> Expr {
        self.mask(|fatty_acids| fatty_acids.is_cis())
    }

    /// Is trans
    #[inline]
    pub fn is_trans(self) -> Expr {
        self.mask(|fatty_acids| fatty_acids.is_trans())
    }
}
