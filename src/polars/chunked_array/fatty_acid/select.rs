use crate::prelude::*;
use polars::prelude::*;

impl FattyAcidChunked {
    #[inline]
    pub fn select(&self, mask: &BooleanChunked, filter: bool) -> PolarsResult<Self> {
        if filter {
            self.filter(mask)
        } else {
            self.nullify(mask)
        }
    }

    #[inline]
    pub fn filter(&self, mask: &BooleanChunked) -> PolarsResult<Self> {
        Ok(Self(self.0.filter(mask)?))
    }

    #[inline]
    pub fn nullify(&self, mask: &BooleanChunked) -> PolarsResult<Self> {
        let null = ListChunked::full_null_with_dtype(PlSmallStr::EMPTY, 1, self.0.inner_dtype());
        Ok(Self(self.0.zip_with(mask, &null)?))
    }

    /// Returns a new [`FattyAcidChunked`] containing only the saturated
    /// elements.
    ///
    /// # Arguments
    ///
    /// * `filter` - If [`true`], removes the original elements; if [`false`],
    ///   replaces the original elements with [`None`].
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing the new [`FattyAcidChunked`] with only
    /// saturated elements.
    #[inline]
    pub fn saturated(&self, filter: bool) -> PolarsResult<Self> {
        self.select(&self.is_saturated()?, filter)
    }

    /// Returns a new [`FattyAcidChunked`] containing only the unsaturated
    /// elements.
    ///
    /// # Arguments
    ///
    /// * `filter` - If [`true`], removes the original elements; if [`false`],
    ///   replaces the original elements with [`None`].
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing the new [`FattyAcidChunked`] with only
    /// unsaturated elements.
    #[inline]
    pub fn unsaturated(&self, filter: bool) -> PolarsResult<Self> {
        self.select(&self.is_unsaturated()?, filter)
    }
}
