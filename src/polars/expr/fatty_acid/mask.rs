use super::FattyAcidExpr;
use crate::prelude::*;
use polars::prelude::*;

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
    pub fn is_unsaturated(self) -> Expr {
        self.mask(|fatty_acids| fatty_acids.is_unsaturated())
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

    #[inline]
    pub fn is_cis(self) -> Expr {
        self.mask(|fatty_acids| fatty_acids.is_cis())
    }

    #[inline]
    pub fn is_trans(self) -> Expr {
        self.mask(|fatty_acids| fatty_acids.is_trans())
    }
}
