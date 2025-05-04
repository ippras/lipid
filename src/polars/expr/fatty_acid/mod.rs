use crate::prelude::*;
use polars::prelude::*;
use polars_ext::prelude::ExprExt;
use std::num::NonZeroI8;

/// Fatty acid [`Expr`]
#[derive(Clone, Debug, PartialEq)]
pub struct FattyAcidExpr(pub Expr);

impl FattyAcidExpr {
    /// Bounds
    #[inline]
    pub fn bounds(self) -> Expr {
        self.0.map(
            |column| Ok(Some(column.try_fatty_acid_list()?.sized()?.into_column())),
            GetOutput::from_type(DataType::UInt8),
        )
    }

    /// Unsaturation
    #[inline]
    pub fn unsaturation(self) -> Expr {
        self.0.map(
            |column| {
                Ok(Some(
                    column.try_fatty_acid_list()?.unsaturation()?.into_column(),
                ))
            },
            GetOutput::from_type(DataType::UInt8),
        )
    }

    /// Fatty acid type (saturated or unsaturated)
    #[inline]
    pub fn r#type(self) -> Expr {
        self.0.map(
            |column| {
                Ok(Some(
                    column.try_fatty_acid_list()?.is_saturated()?.into_column(),
                ))
            },
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
impl<const N: usize> From<&[(Option<Option<NonZeroI8>>, Option<&'static str>); N]>
    for FattyAcidExpr
{
    fn from(value: &[(Option<Option<NonZeroI8>>, Option<&'static str>); N]) -> Self {
        Self(lit(Scalar::new(
            DataType::List(Box::new(FATTY_ACID_DATA_TYPE.clone())),
            AnyValue::List(
                FattyAcidChunked::try_from(value)
                    .unwrap()
                    .into_struct(PlSmallStr::EMPTY)
                    .unwrap()
                    .into_series(),
            ),
        )))
    }
}

#[cfg(feature = "atomic")]
impl Atomic for FattyAcidExpr {
    type Output = Expr;

    /// Returns the number of carbons for each fatty acid in the given series.
    #[inline]
    fn carbons(self) -> Expr {
        self.0.map(
            |column| Ok(Some(column.try_fatty_acid_list()?.carbons()?.into_column())),
            GetOutput::from_type(DataType::UInt8),
        )
    }

    /// Returns the number of hydrogens for each fatty acid in the given series.
    #[inline]
    fn hydrogens(self) -> Expr {
        self.0.map(
            |column| {
                Ok(Some(
                    column.try_fatty_acid_list()?.hydrogens()?.into_column(),
                ))
            },
            GetOutput::from_type(DataType::UInt8),
        )
    }
}

#[cfg(feature = "ecn")]
impl EquivalentCarbonNumber for FattyAcidExpr {
    type Output = Expr;

    #[inline]
    fn equivalent_carbon_number(self) -> Expr {
        self.0.map(
            |column| {
                Ok(Some(
                    column
                        .try_fatty_acid_list()?
                        .equivalent_carbon_number()?
                        .into_column(),
                ))
            },
            GetOutput::from_type(DataType::UInt8),
        )
    }
}

#[cfg(feature = "ecl")]
impl EquivalentChainLength for FattyAcidExpr {
    type Output = Expr;

    #[inline]
    fn equivalent_chain_length(self, retention_time: Expr, logarithmic: bool) -> Expr {
        self.clone()
            .nullify(self.clone().is_saturated())
            .fatty_acid()
            .carbons()
            .forward_fill(None)
            + self.fractional_chain_length(retention_time, logarithmic)
    }

    #[inline]
    fn fractional_chain_length(self, retention_time: Expr, logarithmic: bool) -> Expr {
        let maybe_logarithmic = |mut expr: Expr| {
            if logarithmic {
                expr = expr.log(10.0)
            }
            expr
        };
        let unsaturated_time = || maybe_logarithmic(retention_time.clone());
        let saturated_time =
            || maybe_logarithmic(retention_time.clone().nullify(self.clone().is_saturated()));
        let saturated_carbons = || {
            self.clone()
                .nullify(self.clone().is_saturated())
                .fatty_acid()
                .carbons()
        };
        ternary_expr(
            self.clone().is_saturated(),
            lit(0),
            (saturated_carbons().backward_fill(None) - saturated_carbons().forward_fill(None))
                * (unsaturated_time() - saturated_time().forward_fill(None))
                / (saturated_time().backward_fill(None) - saturated_time().forward_fill(None)),
        )
    }
}

pub mod factor;

mod equal;
mod indices;
mod kind;
#[cfg(feature = "mask")]
mod mask;
#[cfg(feature = "mass")]
mod mass;
#[cfg(feature = "select")]
mod select;
