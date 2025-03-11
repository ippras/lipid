use crate::{polars::bound, prelude::*};
use polars::prelude::*;
use std::iter::zip;

// /// Returns the bounds of each fatty acid in the given series.
// ///
// /// # Arguments
// ///
// /// * `fatty_acids` - A reference to a [`Series`] containing fatty acids.
// ///
// /// # Returns
// ///
// /// A [`PolarsResult`] containing a `UInt8Chunked` series with the bounds of
// /// each fatty acid.
// pub fn bounds(series: &Series) -> PolarsResult<Series> {
//     Ok(series.fatty_acid()?.bounds()?.into_series())
// }

// /// Filters the fatty acids in the given series based on a predicate function.
// ///
// /// # Arguments
// ///
// /// * `fatty_acids` - A reference to a [`Series`] containing fatty acids.
// /// * `predicate` - A function that takes a reference to a [`Series`] and
// ///   returns a [`PolarsResult<bool>`].
// ///
// /// # Returns
// ///
// /// A [`PolarsResult`] containing a `BooleanChunked` series indicating which
// /// fatty acids satisfy the predicate.
// pub fn mask(
//     series: &Series,
//     predicate: impl Fn(&Series) -> PolarsResult<bool>,
// ) -> PolarsResult<BooleanChunked> {
//     series
//         .list()?
//         .into_iter()
//         .map(|bounds| {
//             let Some(bounds) = bounds else {
//                 return Ok(None);
//             };
//             Ok(Some(predicate(&bounds)?))
//         })
//         .collect()
// }

// pub fn mask1(
//     predicate: impl Fn(&Series) -> PolarsResult<bool>,
// ) -> impl Fn(&Series) -> PolarsResult<Series> {
//     move |series| {
//         Ok(series
//             .fatty_acid()?
//             .into_iter()
//             .map(|bounds| {
//                 let Some(bounds) = bounds else {
//                     return Ok(None);
//                 };
//                 Ok(Some(predicate(&bounds)?))
//             })
//             .collect::<PolarsResult<BooleanChunked>>()?
//             .into_series())
//     }
// }

// /// Adds an index to each unsaturated bound in the given series of fatty acids.
// ///
// /// # Arguments
// ///
// /// * `fatty_acids` - A reference to a [`Series`] containing fatty acids.
// ///
// /// # Returns
// ///
// /// A [`PolarsResult`] containing a `ListChunked` series with indexed
// /// unsaturated bounds.
// pub fn unsaturated_indexed(series: &Series) -> PolarsResult<ListChunked> {
//     series
//         .list()?
//         .into_iter()
//         .map(|bounds| {
//             let Some(bounds) = bounds else {
//                 return Ok(None);
//             };
//             let mut indices = PrimitiveChunkedBuilder::<IdxType>::new("Index".into(), bounds.len());
//             let values = bounds
//                 .categorical()?
//                 .iter_str()
//                 .enumerate()
//                 .filter_map(|(index, id)| {
//                     if bound::is_unsaturated(id) {
//                         indices.append_value(index as _);
//                         id
//                     } else {
//                         None
//                     }
//                 })
//                 .collect::<Series>()
//                 .cast(&BOUND_DATA_TYPE)?;
//             Ok(Some(
//                 StructChunked::from_series(
//                     PlSmallStr::EMPTY,
//                     values.len(),
//                     [indices.finish().into_series(), values.into_series()].iter(),
//                 )?
//                 .into_series(),
//             ))
//         })
//         .collect::<PolarsResult<ListChunked>>()
// }

// /// Struct
// /// 0. FattyAcid
// /// 1. MAG2: [`f64`]
// /// 2. TAG: [`f64`]
// pub fn selectivity_factor(series: &Series) -> PolarsResult<Series> {
//     let r#struct = series.struct_()?;
//     let fields = r#struct.fields_as_series();
//     let fatty_acid = fields[0].fatty_acid()?;
//     let mag2 = fields[1].f64()?;
//     let tag = fields[2].f64()?;
//     let mag2_unsaturated_sum = mag2.filter(&fatty_acid.is_unsaturated()?)?.sum();
//     let tag_unsaturated_sum = tag.filter(&fatty_acid.is_unsaturated()?)?.sum();
//     Ok(zip(tag, mag2)
//         .map(|(tag, mag2)| {
//             let Some(tag) = tag else {
//                 return Ok(None);
//             };
//             let Some(mag2) = mag2 else {
//                 return Ok(None);
//             };
//             let Some(mag2_unsaturated_sum) = mag2_unsaturated_sum else {
//                 return Ok(None);
//             };
//             let Some(tag_unsaturated_sum) = tag_unsaturated_sum else {
//                 return Ok(None);
//             };
//             Ok(Some(
//                 (mag2 / tag) / (mag2_unsaturated_sum / tag_unsaturated_sum),
//             ))
//         })
//         .collect::<PolarsResult<Float64Chunked>>()?
//         .into_series())
// }
