use crate::prelude::*;
use polars::prelude::*;

/// Fatty acid chunked array
#[derive(Clone, Debug)]
pub struct FattyAcidChunked<'a>(&'a ListChunked);

impl<'a> FattyAcidChunked<'a> {
    pub fn new(series: &'a Series) -> PolarsResult<Self> {
        let fatty_acids = series.list()?;
        polars_ensure!(
            *fatty_acids.inner_dtype() == *Bound::DATA_TYPE,
            SchemaMismatch: "invalid fatty_acid series inner datatype: expected `Bound`, got = `{}`",
            fatty_acids.inner_dtype(),
        );
        Ok(Self(fatty_acids))
    }
}

impl FattyAcidChunked<'_> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<BoundSeries> {
        Some(BoundSeries(self.0.get_as_series(index)?))
    }

    // /// Return bool chunked array with is unsaturated values
    // pub fn is_unsaturated(&self) -> PolarsResult<BooleanChunked> {
    //     filter(self.0, is_unsaturated)
    // }

    // pub fn unsaturated(&self) -> PolarsResult<ListChunked> {
    //     unsaturated_indexed(self.0)
    // }

    // pub fn unsaturated(&self, index: usize) -> PolarsResult<Option<UnsaturatedSeries>> {
    //     let Some(unsaturated) = self.unsaturated.list()?.get_as_series(index) else {
    //         return Ok(None);
    //     };
    //     Ok(Some(UnsaturatedSeries::new(&unsaturated)?))
    // }
}

// pub fn is_unsaturated(series: &Series) -> PolarsResult<BooleanChunked> {
//     let mut builder = BooleanChunkedBuilder::new(PlSmallStr::EMPTY, series.len());
//     for bounds in series.list()? {
//         match bounds {
//             Some(bounds) => {
//                 let is_unsaturated = bounds.categorical()?.iter_str().any(|id| id != Some(S));
//                 builder.append_value(is_unsaturated)
//             }
//             None => builder.append_null(),
//         }
//     }
//     Ok(builder.finish())
// }

impl IntoIterator for FattyAcidChunked<'_> {
    type Item = Option<BoundSeries>;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(|bounds| Some(BoundSeries(bounds?)))
    }
}
