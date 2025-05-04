use crate::prelude::*;
use polars::prelude::*;
use std::{convert::identity, num::NonZeroI8};

impl FattyAcidListChunked {
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
    pub fn mask(
        &self,
        f: impl Fn(FattyAcidChunked) -> PolarsResult<bool>,
    ) -> PolarsResult<BooleanChunked> {
        self.0
            .into_iter()
            .map(move |item| {
                let Some(series) = item else {
                    return Ok(None);
                };
                Ok(Some(f(series.try_fatty_acid()?)?))
            })
            .collect()
    }
}

impl Mask for FattyAcidListChunked {
    type Output = PolarsResult<BooleanChunked>;

    /// Returns a boolean chunked array indicating which fatty acids are
    /// saturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// saturated fatty acids and [`false`] otherwise.
    #[inline]
    fn is_saturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|fatty_acid| Ok(fatty_acid.is_saturated()?.is_some_and(identity)))
    }

    /// Returns a boolean chunked array indicating which fatty acids are
    /// unsaturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// unsaturated fatty acids and [`false`] otherwise.
    #[inline]
    fn is_unsaturated(&self, index: Option<NonZeroI8>) -> PolarsResult<BooleanChunked> {
        self.mask(|fatty_acid| Ok(fatty_acid.is_unsaturated(index)?.is_some_and(identity)))
    }

    /// Returns a boolean chunked array indicating which fatty acids are
    /// monounsaturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// monounsaturated fatty acids and [`false`] otherwise.
    #[inline]
    fn is_monounsaturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|fatty_acid| Ok(fatty_acid.is_monounsaturated()?.is_some_and(identity)))
    }

    /// Returns a boolean chunked array indicating which fatty acids are
    /// polyunsaturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// polyunsaturated fatty acids and [`false`] otherwise.
    #[inline]
    fn is_polyunsaturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|fatty_acid| Ok(fatty_acid.is_polyunsaturated()?.is_some_and(identity)))
    }

    /// Returns a boolean chunked array indicating which fatty acids are cis.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// fatty acids with unsaturated cis-only bonds and [`false`] otherwise.
    #[inline]
    fn is_cis(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|fatty_acid| Ok(fatty_acid.is_cis()?.is_some_and(identity)))
    }

    /// Returns a boolean chunked array indicating which fatty acids are trans.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// fatty acids with trans bonds and [`false`] otherwise.
    #[inline]
    fn is_trans(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|fatty_acid| Ok(fatty_acid.is_trans()?.is_some_and(identity)))
    }
}
