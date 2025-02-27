pub use self::kind::{Rco, Rcoo, Rcooch3, Rcooh};
use crate::{
    fatty_acid::{FattyAcid, Unsaturated},
    polars::bound::{
        IDENTIFIERS,
        identifiers::{D, DC, DT, S, T, TC, TT},
    },
    prelude::*,
};
use polars::{chunked_array::builder::AnonymousOwnedListBuilder, prelude::*};
use polars_ext::column;

/// Bounds
pub fn bounds(fatty_acids: &Series) -> PolarsResult<UInt8Chunked> {
    Ok(fatty_acids
        .list()?
        .into_iter()
        .map(|bounds| Some(bounds?.len() as _))
        .collect())
}

/// Carbons
pub fn carbons(fatty_acids: &Series) -> PolarsResult<UInt8Chunked> {
    Ok(bounds(fatty_acids)?.apply(|bounds| Some(bounds? + 1)))
}

// pub fn hydrogens(self) -> Expr {
//     lit(2) * self.clone().carbons() - lit(2) * self.unsaturated().sum()
// }
/// Hydrogens
pub fn hydrogens(fatty_acids: &Series) -> PolarsResult<UInt8Chunked> {
    let t = fatty_acids
        .list()?
        .into_iter()
        .map(|bounds| {
            let Some(bounds) = bounds else {
                return Ok(None);
            };
            Ok(Some(
                bounds
                    .categorical()?
                    .iter_str()
                    .filter_map(|id| {
                        let bound = Bound::new(id?);
                        Some(Into::<&str>::into(bound))
                    })
                    .collect::<Series>(),
            ))
        })
        .collect::<PolarsResult<ListChunked>>()?;
    // let unsaturated = unsaturated(fatty_acids)?
    //     .apply_to_inner(&|unsaturated| unsaturated.struct_()?.field_by_name(""))?;
    Ok(carbons(fatty_acids)?.apply(|carbons| Some(2 * carbons? - 2)))
}

/// Filter
pub fn filter(
    fatty_acids: &Series,
    predicate: impl Fn(Option<&str>) -> bool,
) -> PolarsResult<BooleanChunked> {
    fatty_acids
        .list()?
        .into_iter()
        .map(|bounds| {
            let Some(bounds) = bounds else {
                return Ok(None);
            };
            Ok(Some(bounds.categorical()?.iter_str().any(&predicate)))
        })
        .collect()
}

// /// Filters null and unsaturated
// #[inline]
// pub fn is_saturated(id: Option<&str>) -> bool {
//     id.is_some() && id != Some(S)
// }

/// Filters null and saturated
#[inline]
pub fn is_unsaturated(id: Option<&str>) -> bool {
    id.is_some() && id != Some(S)
}

/// Filters saturated
#[inline]
pub fn is_not_saturated(id: Option<&str>) -> bool {
    id != Some(S)
}

pub fn is_unsaturated_g(bounds: &Series) -> bool {
    bounds
        .categorical()
        .map_or(false, |bounds| bounds.iter_str().any(is_unsaturated))

}

pub fn unsaturated(fatty_acids: &Series) -> PolarsResult<ListChunked> {
    fatty_acids
        .list()?
        .into_iter()
        .map(|bounds| {
            let Some(bounds) = bounds else {
                return Ok(None);
            };
            let mut indices = PrimitiveChunkedBuilder::<IdxType>::new("Index".into(), bounds.len());
            let values = bounds
                .categorical()?
                .iter_str()
                .enumerate()
                .filter_map(|(index, id)| {
                    if is_unsaturated(id) {
                        indices.append_value(index as _);
                        id
                    } else {
                        None
                    }
                })
                .collect::<Series>()
                .cast(&Bound::DATA_TYPE)?;
            Ok(Some(
                StructChunked::from_series(
                    PlSmallStr::EMPTY,
                    values.len(),
                    [indices.finish().into_series(), values.into_series()].iter(),
                )?
                .into_series(),
            ))
        })
        .collect::<PolarsResult<ListChunked>>()
}

/// Fatty acid [`Expr`]
#[derive(Clone, Debug)]
pub struct FattyAcidExpr(pub Expr);

impl FattyAcidExpr {
    /// Bounds
    #[inline]
    pub fn bounds(self) -> Expr {
        self.0.map(
            column(|series| Ok(bounds(series)?.into_series())),
            GetOutput::from_type(DataType::UInt8),
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
        self.0.map(
            column(|fatty_acids| Ok(filter(fatty_acids, is_unsaturated)?.into_series())),
            GetOutput::from_type(DataType::Boolean),
        )
    }

    /// Unsaturated
    #[inline]
    pub fn unsaturated(self) -> Expr {
        self.0.map(
            column(|fatty_acids| Ok(unsaturated(fatty_acids)?.into_series())),
            GetOutput::from_type(DataType::List(Box::new(DataType::Struct(vec![
                Field::new("Index".into(), IDX_DTYPE),
                Field::new(PlSmallStr::EMPTY, Bound::DATA_TYPE.clone()),
            ])))),
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
        self.0.map(
            column(|series| Ok(carbons(series)?.into_series())),
            GetOutput::same_type(),
        )
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
