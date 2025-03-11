use crate::prelude::*;
use polars::prelude::*;
use std::num::NonZeroI8;

impl FattyAcidChunked {
    /// Applies a mask function to the chunked array.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes a reference to a [`BoundChunked`] and
    ///   returns a boolean.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with the result of
    /// the mask function.
    #[inline]
    pub fn mask(&self, f: impl Fn(&BoundChunked) -> bool) -> PolarsResult<BooleanChunked> {
        self.0
            .into_iter()
            .map(move |item| {
                let Some(series) = item else {
                    return Ok(None);
                };
                Ok(Some(f(series.bound()?)))
            })
            .collect()
    }

    /// Returns a boolean chunked array indicating which fatty acids are
    /// saturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// saturated fatty acids and [`false`] otherwise.
    #[inline]
    pub fn is_saturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|bounds| bounds.is_saturated())
    }

    /// Returns a boolean chunked array indicating which fatty acids are
    /// unsaturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// unsaturated fatty acids and [`false`] otherwise.
    #[inline]
    pub fn is_unsaturated(&self, n: Option<NonZeroI8>) -> PolarsResult<BooleanChunked> {
        self.mask(|bounds| bounds.is_unsaturated(n))
    }

    /// Returns a boolean chunked array indicating which fatty acids are
    /// monounsaturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// monounsaturated fatty acids and [`false`] otherwise.
    #[inline]
    pub fn is_monounsaturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|bounds| bounds.is_monounsaturated())
    }

    /// Returns a boolean chunked array indicating which fatty acids are
    /// polyunsaturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// polyunsaturated fatty acids and [`false`] otherwise.
    #[inline]
    pub fn is_polyunsaturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|bounds| bounds.is_polyunsaturated())
    }

    /// Returns a boolean chunked array indicating which fatty acids are cis.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// fatty acids with unsaturated cis-only bonds and [`false`] otherwise.
    #[inline]
    pub fn is_cis(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|bounds| bounds.is_cis())
    }

    /// Returns a boolean chunked array indicating which fatty acids are trans.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// fatty acids with trans bonds and [`false`] otherwise.
    #[inline]
    pub fn is_trans(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|bounds| bounds.is_trans())
    }
}
