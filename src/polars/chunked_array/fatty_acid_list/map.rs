use crate::prelude::*;
use polars::prelude::*;

impl FattyAcidListChunked {
    /// Applies a function to each fatty acid in the chunked array and returns
    /// an iterator over the results.
    ///
    /// # Arguments
    ///
    /// * `f` - A function to apply to each fatty acid.
    ///
    /// # Returns
    ///
    /// An iterator over the results of applying the function to each fatty
    /// acid.
    #[inline]
    pub fn map<T>(
        &self,
        f: impl Fn(FattyAcidChunked) -> T,
    ) -> impl Iterator<Item = PolarsResult<Option<T>>> {
        self.0.into_iter().map(move |item| {
            let Some(series) = item else {
                return Ok(None);
            };
            Ok(Some(f(series.try_fatty_acid()?)))
        })
    }

    /// Returns the number of bounds for each fatty acid in the chunked array.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`UInt8Chunked`] with the number of
    /// bounds for each fatty acid.
    pub fn sized(&self) -> PolarsResult<UInt8Chunked> {
        self.map(|fatty_acid| fatty_acid.sized()).collect()
    }

    /// Returns the unsaturation levels of each fatty acid in the chunked array.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`UInt8Chunked`] with the unsaturation
    /// levels.
    pub fn unsaturation(&self) -> PolarsResult<UInt8Chunked> {
        self.map(|fatty_acid| fatty_acid.unsaturation()).collect()
    }
}
