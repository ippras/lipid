use crate::{polars::bound, prelude::*};
use polars::prelude::*;

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

/// Fatty acid contains only saturated bounds.
///
/// [bound::is_saturated]
#[inline]
pub fn is_saturated(bounds: &Series) -> PolarsResult<bool> {
    Ok(bounds.categorical()?.iter_str().all(bound::is_saturated))
}

/// Fatty acid contains only saturated bounds or [`None`].
///
/// [bound::is_not_unsaturated]
#[inline]
pub fn is_not_unsaturated(bounds: &Series) -> PolarsResult<bool> {
    Ok(bounds
        .categorical()?
        .iter_str()
        .all(bound::is_not_unsaturated))
}

/// Fatty acid contains only unsaturated bounds.
///
/// [bound::is_unsaturated]
#[inline]
pub fn is_unsaturated(bounds: &Series) -> PolarsResult<bool> {
    Ok(bounds.categorical()?.iter_str().any(bound::is_unsaturated))
}

/// Fatty acid contains only unsaturated bounds or [`None`].
///
/// [bound::is_not_saturated]
#[inline]
pub fn is_not_saturated(bounds: &Series) -> PolarsResult<bool> {
    Ok(bounds
        .categorical()?
        .iter_str()
        .any(bound::is_not_saturated))
}

/// Add index to each bound
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
