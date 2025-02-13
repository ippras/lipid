pub use self::kind::{Rco, Rcoo, Rcooch3, Rcooh};
use crate::fatty_acid::{FattyAcid, Unsaturated};
use polars::prelude::*;

/// Fatty acid [`Expr`]
#[derive(Clone, Debug)]
pub struct FattyAcidExpr(pub Expr);

impl FattyAcidExpr {
    /// Carbons
    pub fn carbons(self) -> Expr {
        self.0.struct_().field_by_name("Carbons")
    }

    /// Unsaturated
    pub fn unsaturated(self) -> UnsaturatedExpr {
        UnsaturatedExpr(self.0.struct_().field_by_name("Unsaturated"))
    }

    /// Equal
    pub fn equal(self, other: impl Into<FattyAcidExpr>) -> Expr {
        self.0.eq(other.into().0)
    }

    /// Replace unsaturated with null
    pub fn saturated_or_null(self, expr: Expr) -> Expr {
        ternary_expr(self.is_saturated(), expr, lit(NULL))
    }

    /// Replace saturated with null
    pub fn unsaturated_or_null(self, expr: Expr) -> Expr {
        ternary_expr(self.is_saturated().not(), expr, lit(NULL))
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

    // pub fn r#type(self) -> Expr {
    //     ternary_expr(self.saturated(), lit("S"), lit("U"))
    // }

    // pub fn unsaturated(self) -> Expr {
    //     self.saturated().not()
    // }

    // pub fn unsaturation(self) -> Expr {
    //     self.d() + lit(2) * self.t()
    // }
}

impl FattyAcidExpr {
    /// Bounds
    pub fn bounds(self) -> Expr {
        (self.carbons() - lit(1)).clip_min(lit(0))
    }

    /// Hydrogens
    ///
    /// `H = 2C - 2U`
    pub fn hydrogens(self) -> Expr {
        lit(2) * self.clone().carbons() - lit(2) * self.unsaturated().sum()
    }

    /// Is saturated
    pub fn is_saturated(self) -> Expr {
        self.unsaturated().len().eq(0)
    }

    /// Is unsaturated
    pub fn is_unsaturated(self) -> Expr {
        self.is_saturated().not()
    }

    /// [`bounds`]
    pub fn b(self) -> Expr {
        self.bounds()
    }

    /// [`carbons`]
    pub fn c(self) -> Expr {
        self.carbons()
    }

    /// [`hydrogens`]
    pub fn h(self) -> Expr {
        self.hydrogens()
    }

    /// [`saturated`]
    pub fn s(self) -> Expr {
        self.is_saturated()
    }

    /// [`unsaturation`]
    pub fn u(self) -> Expr {
        self.unsaturated().len()
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
    pub fn len(self) -> Expr {
        self.0
            .list()
            .eval(col("").struct_().field_by_name("Unsaturation"), true)
            .list()
            .len()
    }

    /// Unsaturation sum
    pub fn sum(self) -> Expr {
        self.0
            .list()
            .eval(col("").struct_().field_by_name("Unsaturation"), true)
            .list()
            .sum()
    }

    /// List
    pub fn list(self) -> ListNameSpace {
        self.0.list()
    }

    /// Equal
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
