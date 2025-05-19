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

    /// Is saturated
    #[inline]
    pub fn is_saturated(self) -> Expr {
        self.mask(move |fatty_acids| fatty_acids.is_saturated())
    }

    /// Is unsaturated
    #[inline]
    pub fn is_unsaturated(self, offset: Option<NonZeroI8>) -> Expr {
        self.mask(move |fatty_acids| fatty_acids.is_unsaturated(offset))
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

impl From<FattyAcidExpr> for Expr {
    fn from(value: FattyAcidExpr) -> Self {
        value.0
    }
}

impl<T: TryInto<FattyAcidChunked, Error = PolarsError>> From<T> for FattyAcidExpr {
    fn from(value: T) -> Self {
        Self(lit(Scalar::new(
            DataType::List(Box::new(FATTY_ACID_DATA_TYPE.clone())),
            AnyValue::List(
                value
                    .try_into()
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
            .fill_null_with_strategy(FillNullStrategy::Forward(None))
            + self.fractional_chain_length(retention_time, logarithmic)
    }

    #[inline]
    fn fractional_chain_length(self, retention_time: Expr, logarithmic: bool) -> Expr {
        const BASE: f64 = 10.0;

        let maybe_logarithmic = |mut expr: Expr| {
            if logarithmic {
                expr = expr.log(BASE)
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
            (saturated_carbons().fill_null_with_strategy(FillNullStrategy::Backward(None))
                - saturated_carbons().fill_null_with_strategy(FillNullStrategy::Forward(None)))
                * (unsaturated_time()
                    - saturated_time().fill_null_with_strategy(FillNullStrategy::Forward(None)))
                / (saturated_time().fill_null_with_strategy(FillNullStrategy::Backward(None))
                    - saturated_time().fill_null_with_strategy(FillNullStrategy::Forward(None))),
        )
    }
}

pub mod factor;

mod equal;
mod indices;
mod kind;
#[cfg(feature = "mass")]
mod mass;
#[cfg(feature = "select")]
mod select;
