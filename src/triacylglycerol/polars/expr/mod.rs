use crate::fatty_acid::polars::{ExprExt as _, expr::FattyAcidExpr};
use polars::{
    lazy::dsl::{max_horizontal, min_horizontal},
    prelude::*,
};

/// Extension methods for [`Expr`]
pub trait ExprExt: Sized {
    fn triacylglycerol(self) -> TriacylglycerolExpr;

    fn tag(self) -> TriacylglycerolExpr {
        self.triacylglycerol()
    }
}

impl ExprExt for Expr {
    fn triacylglycerol(self) -> TriacylglycerolExpr {
        TriacylglycerolExpr(self)
    }
}

/// Triacylglycerol [`Expr`]
#[derive(Clone, Debug, Hash)]
pub struct TriacylglycerolExpr(Expr);

impl TriacylglycerolExpr {
    pub fn stereospecific_numbers(self) -> Expr {
        self.0.struct_().field_by_name("*")
    }

    pub fn stereospecific_number1(self) -> Expr {
        self.0.struct_().field_by_name("StereospecificNumber1")
    }

    pub fn stereospecific_number2(self) -> Expr {
        self.0.struct_().field_by_name("StereospecificNumber2")
    }

    pub fn stereospecific_number3(self) -> Expr {
        self.0.struct_().field_by_name("StereospecificNumber3")
    }

    pub fn sn(self) -> Expr {
        self.stereospecific_numbers()
    }

    pub fn sn1(self) -> Expr {
        self.stereospecific_number1()
    }

    pub fn sn2(self) -> Expr {
        self.stereospecific_number2()
    }

    pub fn sn3(self) -> Expr {
        self.stereospecific_number3()
    }

    pub fn map(self, f: impl Fn(Expr) -> Expr) -> Expr {
        as_struct(vec![
            f(self.clone().sn1()).alias("StereospecificNumber1"),
            f(self.clone().sn2()).alias("StereospecificNumber2"),
            f(self.sn3()).alias("StereospecificNumber3"),
        ])
    }

    pub fn sum(self) -> Expr {
        self.clone().sn1() + self.clone().sn2() + self.sn3()
    }

    pub fn unsaturation(self) -> Expr {
        self.clone().sn1().fa().unsaturated().sum()
            + self.clone().sn2().fa().unsaturated().sum()
            + self.sn3().fa().unsaturated().sum()
    }

    pub fn test_map_apply<F>(self, f: &'static F) -> Expr
    where
        F: Fn(&Series) -> PolarsResult<Series> + Send + Sync,
    {
        as_struct(vec![
            self.clone()
                .sn1()
                .map(column(f), GetOutput::same_type())
                .alias("StereospecificNumber1"),
            self.clone()
                .sn2()
                .map(column(f), GetOutput::same_type())
                .alias("StereospecificNumber2"),
            self.clone()
                .sn3()
                .map(column(f), GetOutput::same_type())
                .alias("StereospecificNumber3"),
        ])
    }

    pub fn test_positional(self, f: impl Fn(Expr) -> Expr) -> Expr {
        ternary_expr(
            f(self.clone().sn1()).lt_eq(f(self.clone().sn3())),
            as_struct(vec![
                self.clone().sn1(),
                self.clone().sn2(),
                self.clone().sn3(),
            ]),
            as_struct(vec![self.clone().sn3(), self.clone().sn2(), self.sn1()]),
        )
    }

    pub fn test_non_stereospecific(self, f: impl Fn(Expr) -> Expr) -> PolarsResult<Expr> {
        Ok(
            concat_list([self.clone().sn1(), self.clone().sn2(), self.sn3()])?
                .list()
                .eval(col("").sort_by([f(col(""))], Default::default()), true)
                .list()
                .to_struct(ListToStructArgs::FixedWidth(
                    [
                        "StereospecificNumber1".into(),
                        "StereospecificNumber2".into(),
                        "StereospecificNumber3".into(),
                    ]
                    .into(),
                )),
        )
    }

    // pub fn non_stereospecific_a(self, f: impl Fn(Expr) -> Expr) -> Expr {
    //     let function = move |column: Column| {
    //         let sn1 = column.struct_()?.field_by_name("StereospecificNumber1")?;
    //         let sn2 = column.struct_()?.field_by_name("StereospecificNumber2")?;
    //         let sn3 = column.struct_()?.field_by_name("StereospecificNumber3")?;
    //         let mask = sn1
    //             .cast(&DataType::Int64)?
    //             .i64()?
    //             .lt_eq(sn1.cast(&DataType::Int64)?.i64()?);
    //         // let (sn1, sn3) = (sn1.zip_with(&mask, &sn3)?, sn1.zip_with(&!mask, &sn3)?);
    //         let reordered: (Vec<_>, Vec<_>) = sn1
    //             .iter()
    //             .zip(sn3.iter())
    //             .map(|(sn1, sn3)| {
    //                 let sn1 = f(lit(sn1.into()));
    //                 let sn3 = f(lit(sn3.into()));
    //                 if sn1.lt_eq(sn3) {
    //                     (sn1, sn3)
    //                 } else {
    //                     (sn3, sn1)
    //                 }
    //             })
    //             .unzip();
    //         // let sn1 = Series::new(sn1.name().clone(), reordered.0);
    //         // let sn3 = Series::new(sn1.name().clone(), reordered.1);
    //         // let t1 = Series::from_iter(
    //         //     sn1.iter()
    //         //         .zip(sn3.iter())
    //         //         .map(|(sn1, sn3)| if sn1 <= sn3 { sn1 } else { sn3 }),
    //         // );
    //         // let (sn1, sn3) = (
    //         //     sn1.iter()
    //         //         .zip(sn3.iter())
    //         //         .map(|(sn1, sn3)| Ok(Some(if sn1 <= sn3 { sn1 } else { sn3 })))
    //         //         .collect::<PolarsResult<ChunkedArray<_>>>()?,
    //         //     sn1.iter()
    //         //         .zip(sn3.iter())
    //         //         .map(|(sn1, sn3)| Ok(Some(if sn1 <= sn3 { sn3 } else { sn1 })))
    //         //         .collect::<PolarsResult<ChunkedArray<_>>>()?,
    //         // );
    //         for index in 0..column.len() {
    //             // sn1.as_mut();
    //         }
    //         Ok(Some(
    //             StructChunked::from_series(
    //                 column.name().clone(),
    //                 column.len(),
    //                 [sn1.into_series(), sn2, sn3.into_series()].iter(),
    //             )?
    //             .into_column(),
    //         ))
    //     };
    //     self.0
    //         .map(
    //             function,
    //             GetOutput::same_type(),
    //             // GetOutput::map_field(move |f| Ok(eval_field_to_dtype(f, &expr2, true))),
    //         )
    //         .with_fmt("non_stereospecific")
    //     // Ok(
    //     //     concat_list([self.clone().sn1(), self.clone().sn2(), self.sn3()])?
    //     //         .list()
    //     //         .eval(col("").sort_by([f(col(""))], Default::default()), false)
    //     //         .list()
    //     //         .to_struct(ListToStructArgs::FixedWidth(
    //     //             [
    //     //                 "StereospecificNumber1".into(),
    //     //                 "StereospecificNumber2".into(),
    //     //                 "StereospecificNumber3".into(),
    //     //             ]
    //     //             .into(),
    //     //         )),
    //     // )
    // }

    // // Triacylglycerol species
    // pub fn label(self) -> Expr {
    //     concat_str([self.sn().fa().label()], "", true)
    // }

    // Compose
    // pub fn compose(self, composition: Composition) -> PolarsResult<Expr> {
    //     Ok(match composition {
    //         MC => concat_list([
    //             self.sn1().fa().mass(),
    //             self.sn2().fa().mass(),
    //             self.sn3().fa().mass(),
    //         ])?
    //         .list()
    //         .sort(Default::default())
    //         .alias("MC"),
    //         PMC => concat_list([
    //             min_horizontal([self.sn1().fa().mass(), self.sn3().fa().mass()])?,
    //             self.sn2().fa().mass(),
    //             max_horizontal([self.sn1().fa().mass(), self.sn3().fa().mass()])?,
    //         ])?
    //         .alias("PMC"),
    //         SMC => concat_list([
    //             self.sn1().fa().mass(),
    //             self.sn2().fa().mass(),
    //             self.sn3().fa().mass(),
    //         ])?
    //         .alias("SMC"),
    //         NC => concat_list([
    //             self.sn1().fa().ecn(),
    //             self.sn2().fa().ecn(),
    //             self.sn3().fa().ecn(),
    //         ])?
    //         .list()
    //         .sort(Default::default())
    //         .alias("NC"),
    //         PNC => concat_list([
    //             min_horizontal([self.sn1().fa().ecn(), self.sn3().fa().ecn()])?,
    //             self.sn2().fa().ecn(),
    //             max_horizontal([self.sn1().fa().ecn(), self.sn3().fa().ecn()])?,
    //         ])?
    //         .alias("PNC"),
    //         SNC => concat_list([
    //             self.sn1().fa().ecn(),
    //             self.sn2().fa().ecn(),
    //             self.sn3().fa().ecn(),
    //         ])?
    //         .alias("SNC"),
    //         SC => concat_list([self.sn1().fa(), self.sn2().fa(), self.sn3().fa()])?
    //             .list()
    //             .eval(sort_by_species(), true)
    //             .map_list(
    //                 |column| {
    //                     Ok(Some(
    //                         column
    //                             .list()?
    //                             .apply_to_inner(&|series| series.struct_().field_by_name("Label"))?
    //                             .into_series()
    //                             .into_column(),
    //                     ))
    //                 },
    //                 GetOutput::from_type(DataType::List(Box::new(DataType::String))),
    //             )
    //             .alias("SC"),
    //         PSC => {
    //             let sn13 = concat_list([self.sn1().fa(), self.sn3().fa()])?
    //                 .list()
    //                 .eval(sort_by_species(), true);
    //             concat_list([
    //                 sn13.clone().list().get(lit(0), false).fa().label(),
    //                 self.sn2().fa().label(),
    //                 sn13.list().get(lit(1), false).fa().label(),
    //             ])?
    //             .alias("PSC")
    //         }
    //         SSC => concat_list([
    //             self.sn1().fa().label(),
    //             self.sn2().fa().label(),
    //             self.sn3().fa().label(),
    //         ])?
    //         .alias("SSC"),
    //         TC => concat_list([
    //             self.sn1().fa().r#type(),
    //             self.sn2().fa().r#type(),
    //             self.sn3().fa().r#type(),
    //         ])?
    //         .list()
    //         .sort(SortOptions::default())
    //         .alias("TC"),
    //         PTC => concat_list([
    //             min_horizontal([self.sn1().fa().r#type(), self.sn3().fa().r#type()])?,
    //             self.sn2().fa().r#type(),
    //             max_horizontal([self.sn1().fa().r#type(), self.sn3().fa().r#type()])?,
    //         ])?
    //         .alias("PTC"),
    //         STC => concat_list([
    //             self.sn1().fa().r#type(),
    //             self.sn2().fa().r#type(),
    //             self.sn3().fa().r#type(),
    //         ])?
    //         .alias("STC"),
    //         UC => concat_list([
    //             self.sn1().fa().unsaturation(),
    //             self.sn2().fa().unsaturation(),
    //             self.sn3().fa().unsaturation(),
    //         ])?
    //         .list()
    //         .sort(Default::default())
    //         .alias("UC"),
    //         PUC => concat_list([
    //             min_horizontal([
    //                 self.sn1().fa().unsaturation(),
    //                 self.sn3().fa().unsaturation(),
    //             ])?,
    //             self.sn2().fa().unsaturation(),
    //             max_horizontal([
    //                 self.sn3().fa().unsaturation(),
    //                 self.sn1().fa().unsaturation(),
    //             ])?,
    //         ])?
    //         .alias("PUC"),
    //         SUC => concat_list([
    //             self.sn1().fa().unsaturation(),
    //             self.sn2().fa().unsaturation(),
    //             self.sn3().fa().unsaturation(),
    //         ])?
    //         .alias("SUC"),
    //     })
    // }

    // /// Permutate (permutation)
    // pub fn permutate(self, composition: Composition) -> PolarsResult<Expr> {
    //     // let substitution = self.substitute(composition.kind);
    //     // Ok(match composition.stereospecificity {
    //     //     None => {}
    //     //     Some(Stereospecificity::Positional) => {}
    //     //     Some(Stereospecificity::Stereo) => self.0,
    //     // })
    //     Ok(match composition {
    //         MC => concat_list([self.sn1(), self.sn2(), self.sn3()])?
    //             .list()
    //             .eval(
    //                 col("").sort_by([col("").sn().fa().mass()], Default::default()),
    //                 true,
    //             )
    //             .list()
    //             .to_struct(ListToStructArgs::FixedWidth(
    //                 ["SN1".into(), "SN2".into(), "SN3".into()].into(),
    //             )),
    //         NC => concat_list([
    //             min_horizontal([self.sn1().fa().ecn(), self.sn3().fa().ecn()])?,
    //             self.sn2().fa().ecn(),
    //             max_horizontal([self.sn1().fa().ecn(), self.sn3().fa().ecn()])?,
    //         ])?
    //         .list()
    //         .to_struct(ListToStructArgs::FixedWidth(
    //             ["SN1".into(), "SN2".into(), "SN3".into()].into(),
    //         )),
    //         SC => concat_list([self.sn1().fa(), self.sn2().fa(), self.sn3().fa()])?
    //             .list()
    //             .eval(sort_by_species(), true)
    //             .list()
    //             .to_struct(ListToStructArgs::FixedWidth(
    //                 ["SN1".into(), "SN2".into(), "SN3".into()].into(),
    //             )),
    //         PMC => concat_list([
    //             min_horizontal([self.sn1().fa().mass(), self.sn3().fa().mass()])?,
    //             self.sn2().fa().mass(),
    //             max_horizontal([self.sn1().fa().mass(), self.sn3().fa().mass()])?,
    //         ])?,
    //         PNC => concat_list([
    //             min_horizontal([self.sn1().fa().ecn(), self.sn3().fa().ecn()])?,
    //             self.sn2().fa().ecn(),
    //             max_horizontal([self.sn1().fa().ecn(), self.sn3().fa().ecn()])?,
    //         ])?,
    //         PSC => {
    //             let sn13 = concat_list([self.sn1().fa(), self.sn3().fa()])?
    //                 .list()
    //                 .eval(sort_by_species(), true);
    //             concat_list([
    //                 sn13.clone().list().get(lit(0), false).fa().species(),
    //                 self.sn2().fa().species(),
    //                 sn13.list().get(lit(1), false).fa().species(),
    //             ])?
    //         }
    //         PTC => concat_list([
    //             max_horizontal([self.sn1().fa().saturated(), self.sn3().fa().saturated()])?,
    //             self.sn2().fa().saturated(),
    //             min_horizontal([self.sn1().fa().saturated(), self.sn3().fa().saturated()])?,
    //         ])?,
    //         PUC => concat_list([
    //             min_horizontal([
    //                 self.sn1().fa().unsaturation(),
    //                 self.sn3().fa().unsaturation(),
    //             ])?,
    //             self.sn2().fa().unsaturation(),
    //             max_horizontal([
    //                 self.sn1().fa().unsaturation(),
    //                 self.sn3().fa().unsaturation(),
    //             ])?,
    //         ])?,
    //         SMC | SNC | SSC | STC | SUC => self.0,
    //     })
    // }

    // // Substitute (substitution)
    // pub fn substitute(self, kind: Kind) -> Expr {
    //     match kind {
    //         Kind::Ecn => as_struct(vec![
    //             self.sn1().fa().ecn(),
    //             self.sn2().fa().ecn(),
    //             self.sn3().fa().ecn(),
    //         ]),
    //         Kind::Mass => as_struct(vec![
    //             self.sn1().fa().mass(),
    //             self.sn2().fa().mass(),
    //             self.sn3().fa().mass(),
    //         ]),
    //         Kind::Species => as_struct(vec![
    //             as_struct(vec![
    //                 self.sn1().fa().c(),
    //                 self.sn1().fa().d(),
    //                 self.sn1().fa().t(),
    //             ]),
    //             as_struct(vec![
    //                 self.sn2().fa().c(),
    //                 self.sn2().fa().d(),
    //                 self.sn2().fa().t(),
    //             ]),
    //             as_struct(vec![
    //                 self.sn3().fa().c(),
    //                 self.sn3().fa().d(),
    //                 self.sn3().fa().t(),
    //             ]),
    //         ]),
    //         Kind::Type => as_struct(vec![
    //             self.sn1().fa().saturated(),
    //             self.sn2().fa().saturated(),
    //             self.sn3().fa().saturated(),
    //         ]),
    //         Kind::Unsaturation => as_struct(vec![
    //             self.sn1().fa().unsaturation(),
    //             self.sn1().fa().unsaturation(),
    //             self.sn1().fa().unsaturation(),
    //         ]),
    //     }
    // }

    // pub fn composition(expr: Expr, composition: Composition) -> PolarsResult<Expr> {
    //     expr.map(
    //         |column| {
    //             let tag = TriacylglycerolColumn(column);
    //             let sn1 = tag.sn1()?;
    //             let sn3 = tag.sn3()?;
    //             let sn1c = sn1.fa()?.c("FA")?;
    //             // MC => {}
    //             // PMC => {}
    //             // SMC => {}
    //             // NC => {}
    //             // PNC => {}
    //             // SNC => {}
    //             // SC => {}
    //             // PSC => {}
    //             // SSC => {}
    //             // TC => {}
    //             // PTC => {}
    //             // STC => {}
    //             // UC => {}
    //             // PUC => {}
    //             // SUC => {}
    //             match composition.kind {
    //                 Kind::Ecn => {
    //                     sn1.field_by_name("FA");
    //                 }
    //                 Kind::Mass => {}
    //                 Kind::Species => {}
    //                 Kind::Type => {}
    //                 Kind::Unsaturation => {}
    //             };
    //             let out: StructChunked = sn1
    //                 .into_iter()
    //                 .zip(sn3)
    //                 .map(|(sn1, sn3)| match (sn1, sn3) {
    //                     (Some(sn1), Some(sn3)) => Some(a.len() as i32 + b),
    //                     _ => None,
    //                 })
    //                 .collect();
    //             Ok(Some(
    //                 column
    //                     .list()?
    //                     .apply_to_inner(&|series| series.struct_().field_by_name("Label"))?
    //                     .into_series()
    //                     .into_column(),
    //             ))
    //         },
    //         GetOutput::from_type(DataType::List(Box::new(DataType::String))),
    //     );
    //     Ok(lit(0))
    // }

    // pub fn composition(self, composition: Composition) -> PolarsResult<Expr> {
    //     let expr = match composition.kind {
    //         Kind::Ecn => sort_by_ecn(),
    //         Kind::Mass => sort_by_mass(),
    //         Kind::Species => sort_by_species(),
    //         Kind::Type => sort_by_type(),
    //         Kind::Unsaturation => sort_by_unsaturation(),
    //     };
    //     if let Some(Stereospecificity::Stereo) = composition.stereospecificity {
    //         return Ok(self.0);
    //     }
    //     let list = if composition.stereospecificity.is_none() {
    //         concat_list([
    //             self.0.clone().struct_().field_by_name("SN1"),
    //             self.0.clone().struct_().field_by_name("SN2"),
    //             self.0.clone().struct_().field_by_name("SN3"),
    //         ])
    //     } else {
    //         concat_list([
    //             self.0.clone().struct_().field_by_name("SN1"),
    //             self.0.clone().struct_().field_by_name("SN3"),
    //         ])
    //     }?
    //     .list()
    //     .eval(expr, true);
    //     Ok(if composition.stereospecificity.is_none() {
    //         as_struct(vec![
    //             list.clone().list().get(lit(0), false).alias("SN1"),
    //             list.clone().list().get(lit(1), false).alias("SN2"),
    //             list.list().get(lit(2), false).alias("SN3"),
    //         ])
    //     } else {
    //         as_struct(vec![
    //             list.clone().list().get(lit(0), false).alias("SN1"),
    //             self.0.struct_().field_by_name("SN2"),
    //             list.list().get(lit(1), false).alias("SN3"),
    //         ])
    //     })
    // }

    // // Triacylglycerol value
    // pub fn value(self) -> Expr {
    //     self.sn1().value() * self.sn2().value() * self.sn3().value()
    // }
}

impl From<TriacylglycerolExpr> for Expr {
    fn from(value: TriacylglycerolExpr) -> Self {
        value.0
    }
}

pub fn column(
    function: impl Fn(&Series) -> PolarsResult<Series>,
) -> impl Fn(Column) -> PolarsResult<Option<Column>> {
    move |column| {
        let Some(series) = column.as_series() else {
            return Ok(None);
        };
        Ok(Some(function(series)?.into_column()))
    }
}

pub mod chain_length;
pub mod mass;
pub mod permutation;
pub mod stereospecific_number;
