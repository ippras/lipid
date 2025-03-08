use crate::{
    polars::bound::identifiers::{S, U},
    prelude::*,
};
use polars::prelude::*;

/// Fatty acid [`Expr`]
#[derive(Clone, Debug)]
pub struct FattyAcidExpr(pub Expr);

impl FattyAcidExpr {
    /// Bounds
    #[inline]
    pub fn bounds(self) -> Expr {
        self.0.map(
            |column| Ok(Some(column.fatty_acid()?.bounds()?.into_column())),
            GetOutput::from_type(DataType::UInt8),
        )
    }

    /// Unsaturation
    #[inline]
    pub fn unsaturation(self) -> Expr {
        self.0.map(
            |column| Ok(Some(column.fatty_acid()?.unsaturation()?.into_column())),
            GetOutput::from_type(DataType::UInt8),
        )
    }

    /// Fatty acid type (saturated or unsaturated)
    #[inline]
    pub fn r#type(self) -> Expr {
        self.0.map(
            |column| Ok(Some(column.fatty_acid()?.is_saturated()?.into_column())),
            GetOutput::from_type(DataType::UInt8),
        )
        // ternary_expr(self.is_saturated(), lit(S), lit(U)).cast(BOUND_DATA_TYPE.clone())
    }
}

impl From<FattyAcidExpr> for Expr {
    fn from(value: FattyAcidExpr) -> Self {
        value.0
    }
}

impl TryFrom<&[&str]> for FattyAcidExpr {
    type Error = PolarsError;

    fn try_from(value: &[&str]) -> PolarsResult<Self> {
        let series = if value.is_empty() {
            Series::new_empty(PlSmallStr::EMPTY, &BOUND_DATA_TYPE)
        } else {
            Series::from_iter(value.iter().copied()).cast(&BOUND_DATA_TYPE)?
        };
        Ok(FattyAcidExpr(lit(Scalar::new(
            DataType::List(Box::new(BOUND_DATA_TYPE.clone())),
            AnyValue::List(series),
        ))))
    }
}

pub mod r#const;
pub mod factor;
pub mod equal;

mod atomic;
mod chain_length;
mod indices;
mod kind;
mod mask;
mod mass;
mod select;
