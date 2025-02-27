pub use self::kind::{Rco, Rcoo, Rcooch3, Rcooh};
use crate::{
    fatty_acid::{FattyAcid, Unsaturated},
    polars::bound::identifiers::S,
    prelude::*,
};
use polars::{chunked_array::from_iterator_par::ChunkedCollectParIterExt, prelude::*};
use polars_ext::column;
use rayon::iter::ParallelIterator;

pub fn bounds(fatty_acids: &Series) -> PolarsResult<UInt8Chunked> {
    Ok(fatty_acids
        .list()?
        .par_iter()
        .map(|bounds| Some(bounds?.len() as _))
        .collect())
}

pub fn is_unsaturated(fatty_acids: &Series) -> PolarsResult<BooleanChunked> {
    fatty_acids
        .list()?
        .par_iter()
        .map(|bounds| {
            let Some(bounds) = bounds else {
                return Ok(None);
            };
            let is_unsaturated = bounds
                .categorical()?
                .iter_str()
                .any(|id| id.is_some() && id != Some(S));
            Ok(Some(is_unsaturated))
        })
        .collect()
}

// pub fn unsaturated(fatty_acids: &Series) -> PolarsResult<Series> {
//     let name = fatty_acids.name().clone();
//     let mut indices = PrimitiveChunkedBuilder::<UInt32Type>::new("Index".into(), fatty_acids.len());
//     let fatty_acids = fatty_acids
//         .list()?
//         .clone()
//         .par_iter_indexed()
//         .filter_map(|bounds| {
//             (|| {
//                 let Some(bounds) = bounds else {
//                     return Ok(None);
//                 };
//                 let is_unsaturated = bounds
//                     .categorical()?
//                     .iter_str()
//                     .any(|id| id.is_some() && id != Some(S));
//                 if is_unsaturated {
//                     indices.append_value(index as _);
//                     Ok(Some(bounds))
//                 } else {
//                     Ok(None)
//                 }
//             })()
//             .transpose()
//         })
//         .collect::<PolarsResult<ListChunked>>()?;
//     Ok(StructChunked::from_series(
//         name,
//         fatty_acids.len(),
//         [indices.finish().into_series(), fatty_acids.into_series()].iter(),
//     )?
//     .into_series())
// }
pub fn unsaturated(fatty_acids: &Series) -> PolarsResult<Series> {
    let name = fatty_acids.name().clone();
    let fatty_acids: PolarsResult<ListChunked> = fatty_acids
        .list()?
        .clone()
        .par_iter_indexed()
        .filter_map(|bounds| {
            (|| {
                let Some(bounds) = bounds else {
                    return Ok(None);
                };
                let is_unsaturated = bounds
                    .categorical()?
                    .iter_str()
                    .any(|id| id.is_some() && id != Some(S));
                Ok(is_unsaturated.then_some(bounds))
            })()
            .transpose()
        })
        .collect_ca_with_dtype(PlSmallStr::EMPTY, *fatty_acids.dtype());
    Ok(fatty_acids?.into_series())
}

/// Fatty acid [`Expr`]
#[derive(Clone, Debug)]
pub struct FattyAcidExpr(pub Expr);

impl FattyAcidExpr {
    /// Bounds
    #[inline]
    pub fn bounds(self) -> Expr {
        self.0.list().len().cast(DataType::UInt8)
    }

    /// Bounds
    #[inline]
    pub fn bounds_function(self) -> Expr {
        self.0.map(
            column(|series| Ok(bounds(series)?.into_series())),
            GetOutput::from_type(DataType::UInt8),
        )
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

    /// Unsaturated
    #[inline]
    pub fn unsaturated_function(self) -> Expr {
        self.0.map(
            column(|fatty_acids| Ok(unsaturated(fatty_acids)?.into_series())),
            GetOutput::from_type(DataType::Struct(vec![
                Field::new("Index".into(), IDX_DTYPE),
                Field::new(
                    PlSmallStr::EMPTY,
                    DataType::List(Box::new(Bound::DATA_TYPE.clone())),
                ),
            ])),
        )
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

    /// Is unsaturated
    #[inline]
    pub fn is_unsaturated_function(self) -> Expr {
        self.0.map(
            column(|fatty_acids| Ok(is_unsaturated(fatty_acids)?.into_series())),
            GetOutput::from_type(DataType::Boolean),
        )
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

    /// Hydrogens
    ///
    /// `H = 2C - 2U`
    #[inline]
    pub fn hydrogens(self) -> Expr {
        lit(2) * self.clone().carbons() - lit(2) * self.unsaturated().sum()
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
