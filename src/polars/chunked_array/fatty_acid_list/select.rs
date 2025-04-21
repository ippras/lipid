use crate::prelude::*;
use polars::prelude::*;

impl FattyAcidListChunked {
    #[inline]
    pub fn filter(&self, mask: &BooleanChunked) -> PolarsResult<Self> {
        Ok(Self(self.0.filter(mask)?))
    }

    #[inline]
    pub fn nullify(&self, mask: &BooleanChunked) -> PolarsResult<Self> {
        let null = ListChunked::full_null_with_dtype(PlSmallStr::EMPTY, 1, self.0.inner_dtype());
        Ok(Self(self.0.zip_with(mask, &null)?))
    }
}
