use crate::prelude::*;
use polars::prelude::*;

/// Fatty acid [`Expr`]
#[derive(Clone, Debug, PartialEq)]
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

// let series = if value.is_empty() {
//     Series::new_empty(PlSmallStr::EMPTY, &BOUND_DATA_TYPE)
// } else {
//     Series::from_iter(value).cast(&BOUND_DATA_TYPE)?
// };
impl<const N: usize> From<FattyAcid<'_, N>> for FattyAcidExpr {
    fn from(value: FattyAcid<N>) -> Self {
        Self(lit(Scalar::new(
            DataType::List(Box::new(BOUND_DATA_TYPE.clone())),
            AnyValue::List(Series::from_iter(value).cast(&BOUND_DATA_TYPE).unwrap()),
        )))
    }
}

impl<const N: usize> TryFrom<[&str; N]> for FattyAcidExpr {
    type Error = PolarsError;

    fn try_from(value: [&str; N]) -> PolarsResult<Self> {
        Ok(Self(lit(Scalar::new(
            DataType::List(Box::new(BOUND_DATA_TYPE.clone())),
            AnyValue::List(Series::from_iter(value).cast(&BOUND_DATA_TYPE)?),
        ))))
    }
}

pub mod factor;

#[cfg(feature = "atomic")]
mod atomic;
mod chain_length;
mod equal;
mod indices;
mod kind;
#[cfg(feature = "mask")]
mod mask;
#[cfg(feature = "mass")]
mod mass;
#[cfg(feature = "select")]
mod select;
