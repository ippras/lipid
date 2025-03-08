use crate::prelude::*;
use polars::prelude::*;

impl FattyAcidChunked {
    #[inline]
    pub fn mask(&self, f: impl Fn(&BoundChunked) -> bool) -> PolarsResult<BooleanChunked> {
        self.0
            .into_iter()
            .map(move |bounds| {
                let Some(bounds) = bounds else {
                    return Ok(None);
                };
                Ok(Some(f(bounds.bound()?)))
            })
            .collect()
    }

    /// Returns a boolean chunked array indicating which elements are saturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// saturated elements and [`false`] otherwise.
    #[inline]
    pub fn is_saturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|bounds| bounds.is_saturated())
    }

    /// Returns a boolean chunked array indicating which elements are
    /// unsaturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// unsaturated elements and [`false`] otherwise.
    #[inline]
    pub fn is_unsaturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|bounds| bounds.is_unsaturated())
    }

    #[inline]
    pub fn is_monounsaturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|bounds| bounds.is_monounsaturated())
    }

    #[inline]
    pub fn is_polyunsaturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|bounds| bounds.is_polyunsaturated())
    }
}
