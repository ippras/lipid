pub use self::kind::{Rco, Rcoo, Rcooch3, Rcooh};
use crate::{
    fatty_acid::{FattyAcid, Unsaturated},
    polars::bound::identifiers::S,
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
        self.0.list().len().cast(DataType::UInt8)
    }

    /// [`Self::bounds`]
    #[inline]
    pub fn b(self) -> Expr {
        self.bounds()
    }

    /// Saturated
    #[inline]
    pub fn saturated(self) -> Expr {
        self.0.list().eval(
            as_struct(vec![
                col("")
                    .cum_count(false)
                    .filter(col("").eq(lit(S)))
                    .alias("Index"),
                col("").filter(col("").eq(lit(S))),
            ]),
            true,
        )
    }

    /// [`Self::saturated`]
    #[inline]
    pub fn s(self) -> Expr {
        self.saturated()
    }

    /// Unsaturated
    #[inline]
    pub fn unsaturated(self) -> Expr {
        self.0.list().eval(
            as_struct(vec![
                col("")
                    .cum_count(false)
                    .filter(col("").neq(lit(S)))
                    .alias("Index"),
                col("").filter(col("").neq(lit(S))),
            ]),
            true,
        )
    }

    /// [`Self::unsaturated`]
    #[inline]
    pub fn u(self) -> Expr {
        self.unsaturated()
    }

    /// Is saturated
    #[inline]
    pub fn is_saturated(self) -> Expr {
        self.unsaturated().list().len().eq(0)
    }

    /// Is unsaturated
    #[inline]
    pub fn is_unsaturated(self) -> Expr {
        self.is_saturated().not()
    }

    /// Equal
    #[inline]
    pub fn equal(self, other: impl Into<FattyAcidExpr>) -> Expr {
        self.0.eq(other.into().0)
    }

    /// Replace unsaturated with null
    #[inline]
    pub fn saturated_or_null(self, expr: Expr) -> Expr {
        ternary_expr(self.is_saturated(), expr, lit(NULL))
    }

    /// Replace saturated with null
    #[inline]
    pub fn unsaturated_or_null(self, expr: Expr) -> Expr {
        ternary_expr(self.is_unsaturated(), expr, lit(NULL))
    }

    /// Fatty acid type (saturated or unsaturated)
    #[inline]
    pub fn r#type(self) -> Expr {
        ternary_expr(self.is_saturated(), lit(Type::S), lit(Type::U)).cast(Type::DATA_TYPE.clone())
    }

    // /// Double bounds count
    // pub fn d(&self) -> Expr {
    //     self.0
    //         .clone()
    //         .struct_()
    //         .field_by_name("Doubles")
    //         .list()
    //         .len()
    // }

    // /// Triple bounds count
    // pub fn t(&self) -> Expr {
    //     self.0
    //         .clone()
    //         .struct_()
    //         .field_by_name("Triples")
    //         .list()
    //         .len()
    // }
}

/// Atomic methods
impl FattyAcidExpr {
    /// Carbons
    #[inline]
    pub fn carbons(self) -> Expr {
        self.bounds() + lit(1)
    }

    /// [`Self::carbons`]
    #[inline]
    pub fn c(self) -> Expr {
        self.carbons()
    }

    /// Hydrogens
    ///
    /// `H = 2C - 2U`
    #[inline]
    pub fn hydrogens(self) -> Expr {
        lit(2) * self.clone().carbons() - lit(2) * self.unsaturated().sum()
    }

    /// [`Self::hydrogens`]
    #[inline]
    pub fn h(self) -> Expr {
        self.hydrogens()
    }
}

impl From<FattyAcidExpr> for Expr {
    fn from(value: FattyAcidExpr) -> Self {
        value.0
    }
}

impl From<&FattyAcid> for FattyAcidExpr {
    fn from(value: &FattyAcid) -> Self {
        let unsaturated = if value.unsaturated.is_empty() {
            lit(Scalar::new(
                DataType::List(Box::new(DataType::Null)),
                AnyValue::List(Series::new_empty(PlSmallStr::EMPTY, &DataType::Null)),
            ))
        } else {
            concat_list(
                value
                    .unsaturated
                    .iter()
                    .map(
                        |Unsaturated {
                             index,
                             isomerism,
                             unsaturation,
                         }| {
                            as_struct(vec![
                                index.map_or(lit(NULL), lit).alias("Index"),
                                isomerism
                                    .map_or(lit(NULL), |isomerism| lit(isomerism as i8))
                                    .alias("Isomerism"),
                                unsaturation
                                    .map_or(lit(NULL), |unsaturation| lit(unsaturation as u8))
                                    .alias("Unsaturation"),
                            ])
                        },
                    )
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        };
        FattyAcidExpr(as_struct(vec![
            lit(value.carbons).alias("Carbons"),
            unsaturated.alias("Unsaturated"),
        ]))
        // let carbons = lit(value.carbons);
        // let unsaturated = lit(Scalar::new(
        //     DataType::List(Box::new(DataType::Struct(vec![
        //         Field::new("Index".into(), DataType::List(Box::new(DataType::UInt8))),
        //         Field::new("Isomerism".into(), DataType::List(Box::new(DataType::Int8))),
        //         Field::new(
        //             "Unsaturation".into(),
        //             DataType::List(Box::new(DataType::UInt8)),
        //         ),
        //     ]))),
        //     AnyValue::List(Series::from_iter(&value.carbons)),
        // ));
        // todo!()
    }
    // let bounds = Scalar::new(
    //     DataType::List(Box::new(DataType::Int8)),
    //     AnyValue::List(Series::from_iter(&fatty_acid.bounds)),
    // );
}

/// Unsaturated [`Expr`]
#[derive(Clone, Debug)]
pub struct UnsaturatedExpr(pub Expr);

impl UnsaturatedExpr {
    /// Unsaturation length
    ///
    /// The number of unsaturated bonds.
    #[inline]
    pub fn len(self) -> Expr {
        self.0
            .list()
            .eval(col("").struct_().field_by_name("Unsaturation"), true)
            .list()
            .len()
    }

    /// Unsaturation sum
    #[inline]
    pub fn sum(self) -> Expr {
        self.0
            .list()
            .eval(col("").struct_().field_by_name("Unsaturation"), true)
            .list()
            .sum()
    }

    /// List
    #[inline]
    pub fn list(self) -> ListNameSpace {
        self.0.list()
    }

    /// Equal
    #[inline]
    pub fn equal(self, other: UnsaturatedExpr) -> Expr {
        self.0.eq(other)
    }
}

impl From<UnsaturatedExpr> for Expr {
    fn from(value: UnsaturatedExpr) -> Self {
        value.0
    }
}

pub mod r#const;
pub mod factor;
pub mod filter;
pub mod find;
pub mod kind;
pub mod short;

mod chain_length;
mod mass;
