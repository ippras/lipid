use self::unsaturated::UnsaturatedSeries;
use crate::{fatty_acid::FattyAcid, polars::bound::identifiers::S, prelude::Bound};
use polars::prelude::*;
use std::{
    fmt::{self, Display, Formatter},
    iter::from_fn,
};

/// Fatty acid series
#[derive(Clone, Debug)]
pub struct FattyAcidSeries<'a> {
    pub fatty_acids: &'a Series,
}

impl<'a> FattyAcidSeries<'a> {
    pub fn new(series: &'a Series) -> PolarsResult<Self> {
        let _ = series.list()?;
        Ok(Self {
            fatty_acids: series,
        })
    }
}

impl FattyAcidSeries<'_> {
    pub fn len(&self) -> usize {
        self.fatty_acids.len()
    }

    // pub fn get(&self, index: usize) -> PolarsResult<Option<BoundSeries>> {
    //     let Some(carbons) = self.fatty_acids.u8()?.get(index) else {
    //         return Ok(None);
    //     };
    //     let mut unsaturated = Vec::new();
    //     if let Some(series) = self.unsaturated.list()?.get_as_series(index) {
    //         let unsaturated_series = UnsaturatedSeries::new(&series)?;
    //         for index in 0..unsaturated_series.len() {
    //             unsaturated.push(unsaturated_series.get(index)?);
    //         }
    //     };
    //     Ok(Some(FattyAcid {
    //         carbons,
    //         unsaturated,
    //     }))
    // }

    /// Return bool chunked array with is unsaturated values
    pub fn is_unsaturated(&self) -> PolarsResult<BooleanChunked> {
        is_unsaturated(self.fatty_acids)
    }

    // pub fn unsaturated(&self, index: usize) -> PolarsResult<Option<UnsaturatedSeries>> {
    //     let Some(unsaturated) = self.unsaturated.list()?.get_as_series(index) else {
    //         return Ok(None);
    //     };
    //     Ok(Some(UnsaturatedSeries::new(&unsaturated)?))
    // }
}

impl IntoIterator for FattyAcidSeries<'_> {
    type Item = Option<BoundSeries>;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.fatty_acids
            .list()
            .unwrap()
            .into_iter()
            .map(|bounds| Some(BoundSeries { bounds: bounds? }))
    }
}

pub fn is_unsaturated(series: &Series) -> PolarsResult<BooleanChunked> {
    let mut builder = BooleanChunkedBuilder::new(PlSmallStr::EMPTY, series.len());
    for bounds in series.list()? {
        match bounds {
            Some(bounds) => {
                let is_unsaturated = bounds.categorical()?.iter_str().any(|id| id != Some(S));
                builder.append_value(is_unsaturated)
            }
            None => builder.append_null(),
        }
    }
    Ok(builder.finish())
}

/// Bound series
#[derive(Clone, Debug)]
pub struct BoundSeries {
    pub bounds: Series,
}

impl Display for BoundSeries {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let categorical = self.bounds.categorical().unwrap();
        for i in categorical
            .iter_str()
            .map(|bound| Some(Bound::try_from(bound?)))
        {}
        // categorical.to_enum(categories, hash)
        writeln!(f, "")
    }
}

mod unsaturated;
