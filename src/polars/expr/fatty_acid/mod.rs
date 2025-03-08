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

    // /// Unsaturated
    // #[inline]
    // pub fn unsaturated(self) -> Expr {
    //     self.0.map(
    //         column(|fatty_acids| Ok(unsaturated_indexed(fatty_acids)?.into_series())),
    //         GetOutput::from_type(DataType::List(Box::new(DataType::Struct(vec![
    //             Field::new("Index".into(), IDX_DTYPE),
    //             Field::new(PlSmallStr::EMPTY, BOUND_DATA_TYPE.clone()),
    //         ])))),
    //     )
    // }

    /// Equal
    #[inline]
    pub fn equal(self, other: impl Into<FattyAcidExpr>) -> Expr {
        self.0.eq(other.into().0)
    }

    // /// Replace unsaturated with null
    // #[inline]
    // pub fn saturated(self, filter: bool) -> Expr {
    //     self.0.map(
    //         move |column| Ok(Some(column.fatty_acid()?.saturated(filter)?.into_column())),
    //         GetOutput::same_type(),
    //     )
    // }

    // /// Replace saturated with null
    // #[inline]
    // pub fn unsaturated(self, filter: bool) -> Expr {
    //     self.0.map(
    //         move |column| {
    //             Ok(Some(
    //                 column.fatty_acid()?.unsaturated(filter)?.into_column(),
    //             ))
    //         },
    //         GetOutput::same_type(),
    //     )
    // }

    // /// Unsaturated
    // #[inline]
    // pub fn unsaturated(self) -> Expr {
    //     self.0.map(
    //         column(|fatty_acids| Ok(unsaturated_indexed(fatty_acids)?.into_series())),
    //         GetOutput::from_type(DataType::List(Box::new(DataType::Struct(vec![
    //             Field::new("Index".into(), IDX_DTYPE),
    //             Field::new(PlSmallStr::EMPTY, BOUND_DATA_TYPE.clone()),
    //         ])))),
    //     )
    // }

    // /// Replace saturated with null
    // #[inline]
    // pub fn unsaturated_or_null(self, expr: Expr) -> Expr {
    //     ternary_expr(self.is_unsaturated(), expr, lit(NULL))
    // }

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
pub mod find;

mod atomic;
mod chain_length;
mod indices;
mod kind;
mod mask;
mod mass;
mod select;
