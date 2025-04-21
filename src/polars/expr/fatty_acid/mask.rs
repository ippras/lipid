use crate::prelude::*;
use polars::prelude::*;
use std::num::{NonZeroI8, NonZeroU8};

impl FattyAcidExpr {
    #[inline]
    pub fn mask(
        self,
        f: impl Fn(&FattyAcidListChunked) -> PolarsResult<BooleanChunked> + Send + Sync + 'static,
    ) -> Expr {
        self.0.map(
            move |column| Ok(Some(f(column.try_fatty_acid_list()?)?.into_column())),
            GetOutput::from_type(DataType::Boolean),
        )
    }
}

impl Mask for FattyAcidExpr {
    type Output = Expr;

    /// Is saturated
    #[inline]
    fn is_saturated(self) -> Expr {
        self.mask(move |fatty_acids| fatty_acids.is_saturated())
    }

    /// Is unsaturated
    #[inline]
    fn is_unsaturated(self) -> Expr {
        self.mask(move |fatty_acids| fatty_acids.is_unsaturated())
    }

    /// Is monounsaturated
    #[inline]
    fn is_monounsaturated(self) -> Expr {
        self.mask(|fatty_acids| fatty_acids.is_monounsaturated())
    }

    /// Is polyunsaturated
    #[inline]
    fn is_polyunsaturated(self) -> Expr {
        self.mask(|fatty_acids| fatty_acids.is_polyunsaturated())
    }

    /// Is cis
    #[inline]
    fn is_cis(self) -> Expr {
        self.mask(|fatty_acids| fatty_acids.is_cis())
    }

    /// Is trans
    #[inline]
    fn is_trans(self) -> Expr {
        self.mask(|fatty_acids| fatty_acids.is_trans())
    }
}

impl MaskExt for FattyAcidExpr {
    fn try_unsaturated(self, index: Option<NonZeroI8>) -> Expr {
        self.mask(move |fatty_acid| fatty_acid.try_unsaturated(index))
    }

    fn is_delta_unsaturated(self, index: NonZeroU8) -> Expr {
        self.mask(move |fatty_acids| fatty_acids.is_delta_unsaturated(index))
    }

    fn is_omega_unsaturated(self, index: NonZeroU8) -> Expr {
        self.mask(move |fatty_acids| fatty_acids.is_omega_unsaturated(index))
    }
}
