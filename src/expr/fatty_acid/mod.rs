use crate::prelude::*;
use polars::{chunked_array::builder::AnonymousOwnedListBuilder, prelude::*};
use polars_ext::prelude::ExprExt;
use std::num::NonZeroI8;

/// Fatty acid indices column name
pub const INDICES: &str = "Indices";
/// Fatty acid carbon column name
pub const CARBON: &str = "Carbon";
/// Fatty acid column name
pub const FATTY_ACID: &str = "FattyAcid";
/// Fatty acid bound index column name
pub const INDEX: &str = "Index";
/// Fatty acid bound parity column name
pub const PARITY: &str = "Parity";
/// Fatty acid bound triple column name
pub const TRIPLE: &str = "Triple";

/// Fatty acid [`Expr`]
#[derive(Clone, Debug, PartialEq)]
pub struct FattyAcidExpr(pub Expr);

impl FattyAcidExpr {
    /// Unsaturated
    #[inline]
    pub fn indices(self) -> Expr {
        self.0.struct_().field_by_name(INDICES)
    }
}

impl FattyAcidExpr {
    /// Fatty acid type (saturated or unsaturated)
    #[inline]
    pub fn r#type(self) -> Expr {
        ternary_expr(self.is_saturated(), lit("S"), lit("U"))
    }

    /// Double bounds unsaturation
    #[inline]
    pub fn double_bounds_unsaturation(self) -> Expr {
        self.indices()
            .list()
            .eval(element().struct_().field_by_name(TRIPLE).not())
            .list()
            .sum()
    }

    /// Unsaturation
    #[inline]
    pub fn unsaturation(self) -> Expr {
        self.indices()
            .list()
            .eval(
                element()
                    .struct_()
                    .field_by_name(TRIPLE)
                    .cast(DataType::UInt8)
                    + lit(1),
            )
            .list()
            .sum()
            .cast(DataType::UInt8)
    }
}

impl From<FattyAcidExpr> for Expr {
    fn from(value: FattyAcidExpr) -> Self {
        value.0
    }
}

impl From<AnyValue<'static>> for FattyAcidExpr {
    fn from(value: AnyValue<'static>) -> Self {
        Self(lit(Scalar::new(data_type!(FATTY_ACID), value)))
    }
}

impl TryFrom<&FattyAcid<u8, Vec<Unsaturated<Option<u8>, Option<bool>, Option<bool>>>>>
    for FattyAcidExpr
{
    type Error = PolarsError;

    fn try_from(
        value: &FattyAcid<u8, Vec<Unsaturated<Option<u8>, Option<bool>, Option<bool>>>>,
    ) -> Result<Self, Self::Error> {
        let length = value.unsaturated.len();
        let mut index =
            PrimitiveChunkedBuilder::<UInt8Type>::new(PlSmallStr::from_static(INDEX), length);
        let mut triple = BooleanChunkedBuilder::new(PlSmallStr::from_static(TRIPLE), length);
        let mut parity = BooleanChunkedBuilder::new(PlSmallStr::from_static(PARITY), length);
        for unsaturated in &value.unsaturated {
            index.append_option(unsaturated.index);
            triple.append_option(unsaturated.triple);
            parity.append_option(unsaturated.parity);
        }
        let indices = StructChunked::from_series(
            PlSmallStr::EMPTY,
            length,
            [
                index.finish().into_series(),
                triple.finish().into_series(),
                parity.finish().into_series(),
            ]
            .iter(),
        )?;
        Ok(Self::from(AnyValue::StructOwned(Box::new((
            vec![
                AnyValue::UInt8(value.carbon),
                AnyValue::List(indices.into_series()),
            ],
            vec![field!(CARBON), field!(INDICES)],
        )))))
    }
}

#[cfg(feature = "atomic")]
impl Atomic for FattyAcidExpr {
    type Output = Expr;

    #[inline]
    fn carbon(self) -> Expr {
        self.0.struct_().field_by_name(CARBON)
    }

    #[inline]
    fn hydrogen(self) -> Expr {
        self.clone().carbon() * lit(2) - self.unsaturation() * lit(2)
    }

    #[inline]
    fn oxygen(self) -> Expr {
        lit(2)
    }
}

#[cfg(feature = "ecn")]
impl EquivalentCarbonNumber for FattyAcidExpr {
    type Output = Expr;

    #[inline]
    fn equivalent_carbon_number(self) -> Expr {
        self.clone().carbon() - self.unsaturation() * lit(2)
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
            .carbon()
            .fill_null_with_strategy(FillNullStrategy::Forward(None))
            + self.fractional_chain_length(retention_time, logarithmic)
    }

    #[inline]
    fn fractional_chain_length(self, retention_time: Expr, logarithmic: bool) -> Expr {
        const BASE: f64 = 10.0;

        let maybe_logarithmic = |mut expr: Expr| {
            if logarithmic {
                expr = expr.log(lit(BASE))
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
                .carbon()
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

impl FattyAcidTrait for FattyAcidExpr {
    type Expr = Expr;
}

mod display;
mod equal;
mod factors;
mod indices;
mod kind;
mod mask;
pub(crate) mod properties;
#[cfg(feature = "mass")]
mod relative_atomic_mass;
#[cfg(feature = "select")]
mod select;
mod sum;
