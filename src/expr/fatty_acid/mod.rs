use crate::prelude::*;
use polars::prelude::*;
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
    /// Indices
    #[inline]
    pub fn indices(self) -> Expr {
        self.0.struct_().field_by_name(INDICES)
    }

    #[inline]
    pub fn format(self) -> Expr {
        // let indices = self.indices();
        // let carbon = self.carbon();
        // let unsaturated = indices.list().len();
        // format_str("{}:{}-{}", [carbon, unsaturated, indices])
        self.0.map(
            |column| Ok(column.try_fatty_acid()?.delta()?.into_column()),
            |_, field| Ok(Field::new(field.name().clone(), DataType::String)),
        )
    }
}

impl FattyAcidExpr {
    /// Is saturated
    #[inline]
    pub fn is_saturated(self) -> Expr {
        self.indices().list().len().eq(0).alias("IsSaturated")
    }

    /// Is unsaturated
    #[inline]
    pub fn is_unsaturated(self, offset: Option<NonZeroI8>) -> Expr {
        let indices = self.clone().indices().list();
        match offset {
            Some(offset) => match offset.get() {
                omega @ ..0 => {
                    let last = indices.last().struct_().field_by_name(INDEX);
                    last.eq_missing(self.carbon() - lit(omega.unsigned_abs()))
                        .alias(format!("IsUnsaturated{omega}"))
                }
                delta @ 0.. => {
                    let first = indices.first().struct_().field_by_name(INDEX);
                    first
                        .eq_missing(delta)
                        .alias(format!("IsUnsaturated{delta}"))
                }
            },
            None => indices.len().neq(0).alias("IsUnsaturated"),
        }
    }

    /// Is monounsaturated
    #[inline]
    pub fn is_monounsaturated(self) -> Expr {
        self.indices().list().len().eq(1).alias("IsMonounsaturated")
    }

    /// Is polyunsaturated
    #[inline]
    pub fn is_polyunsaturated(self) -> Expr {
        self.indices().list().len().gt(1).alias("IsPolyunsaturated")
    }

    /// Is cis
    #[inline]
    pub fn is_cis(self) -> Expr {
        self.clone()
            .indices()
            .list()
            .len()
            .gt(0)
            .and(
                self.indices()
                    .list()
                    .eval(col("").struct_().field_by_name(PARITY))
                    .list()
                    .any()
                    .not(),
            )
            .alias("IsCis")
    }

    /// Is trans
    #[inline]
    pub fn is_trans(self) -> Expr {
        self.indices()
            .list()
            .eval(col("").struct_().field_by_name(PARITY))
            .list()
            .any()
            .alias("IsTrans")
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
            .eval(col("").struct_().field_by_name(TRIPLE).not())
            .list()
            .sum()
    }

    /// Unsaturation
    #[inline]
    pub fn unsaturation(self) -> Expr {
        self.indices()
            .list()
            .eval(
                col("")
                    .struct_()
                    .field_by_name(TRIPLE)
                    .cast(DataType::UInt8)
                    + lit(1),
            )
            .list()
            .sum()
            .cast(DataType::UInt8)
            .alias("Unsaturation")
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

#[cfg(feature = "atomic")]
impl Atomic for FattyAcidExpr {
    type Output = Expr;

    #[inline]
    fn carbon(self) -> Expr {
        self.0.struct_().field_by_name(CARBON)
    }

    #[inline]
    fn hydrogen(self) -> Expr {
        (self.clone().carbon() * lit(2) - self.unsaturation() * lit(2)).alias("Hydrogen")
    }

    #[inline]
    fn oxygen(self) -> Expr {
        lit(2).alias("Oxygen")
    }
}

#[cfg(feature = "ecn")]
impl EquivalentCarbonNumber for FattyAcidExpr {
    type Output = Expr;

    #[inline]
    fn equivalent_carbon_number(self) -> Expr {
        (self.clone().carbon() - self.unsaturation() * lit(2)).alias("EquivalentCarbonNumber")
    }
}

#[cfg(feature = "ecl")]
impl EquivalentChainLength for FattyAcidExpr {
    type Output = Expr;

    #[inline]
    fn equivalent_chain_length(self, retention_time: Expr, logarithmic: bool) -> Expr {
        (self
            .clone()
            .nullify(self.clone().is_saturated())
            .fatty_acid()
            .carbon()
            .fill_null_with_strategy(FillNullStrategy::Forward(None))
            + self.fractional_chain_length(retention_time, logarithmic))
        .alias("EquivalentChainLength")
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
        .alias("FractionalChainLength")
    }
}

mod equal;
mod factor;
mod indices;
mod kind;
#[cfg(feature = "mass")]
mod mass;
#[cfg(feature = "select")]
mod select;
