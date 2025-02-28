use crate::{polars::bound, prelude::*};
use polars::prelude::*;

/// Returns the bounds of each fatty acid in the given series.
///
/// # Arguments
///
/// * `fatty_acids` - A reference to a [`Series`] containing fatty acids.
///
/// # Returns
///
/// A [`PolarsResult`] containing a `UInt8Chunked` series with the bounds of
/// each fatty acid.
pub fn bounds(fatty_acids: &Series) -> PolarsResult<UInt8Chunked> {
    Ok(fatty_acids
        .list()?
        .into_iter()
        .map(|bounds| Some(bounds?.len() as _))
        .collect())
}

/// Returns the number of carbons for each fatty acid in the given series.
///
/// # Arguments
///
/// * `fatty_acids` - A reference to a [`Series`] containing fatty acids.
///
/// # Returns
///
/// A [`PolarsResult`] containing a `UInt8Chunked` series with the number of
/// carbons for each fatty acid.
pub fn carbons(fatty_acids: &Series) -> PolarsResult<UInt8Chunked> {
    Ok(bounds(fatty_acids)?.apply(|bounds| Some(bounds? + 1)))
}

// pub fn hydrogens(self) -> Expr { lit(2) * self.clone().carbons() - lit(2) *
//     self.unsaturated().sum() }

/// Returns the number of hydrogens for each fatty acid in the given series.
///
/// # Arguments
///
/// * `fatty_acids` - A reference to a [`Series`] containing fatty acids.
///
/// # Returns
///
/// A [`PolarsResult`] containing a `UInt8Chunked` series with the number of
/// hydrogens for each fatty acid.
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
    //     .apply_to_inner(&|unsaturated|
    //     unsaturated.struct_()?.field_by_name(""))?;
    Ok(carbons(fatty_acids)?.apply(|carbons| Some(2 * carbons? - 2)))
}

/// Filters the fatty acids in the given series based on a predicate function.
///
/// # Arguments
///
/// * `fatty_acids` - A reference to a [`Series`] containing fatty acids.
/// * `predicate` - A function that takes a reference to a [`Series`] and
///   returns a [`PolarsResult<bool>`].
///
/// # Returns
///
/// A [`PolarsResult`] containing a `BooleanChunked` series indicating which
/// fatty acids satisfy the predicate.
pub fn filter(
    fatty_acids: &Series,
    predicate: impl Fn(&Series) -> PolarsResult<bool>,
) -> PolarsResult<BooleanChunked> {
    fatty_acids
        .list()?
        .into_iter()
        .map(|bounds| {
            let Some(bounds) = bounds else {
                return Ok(None);
            };
            Ok(Some(predicate(&bounds)?))
        })
        .collect()
}

/// Adds an index to each unsaturated bound in the given series of fatty acids.
///
/// # Arguments
///
/// * `fatty_acids` - A reference to a [`Series`] containing fatty acids.
///
/// # Returns
///
/// A [`PolarsResult`] containing a `ListChunked` series with indexed
/// unsaturated bounds.
pub fn unsaturated_indexed(fatty_acids: &Series) -> PolarsResult<ListChunked> {
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
                    if bound::is_unsaturated(id) {
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
