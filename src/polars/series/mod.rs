use crate::polars::chunked_array::{BoundChunked, FattyAcidChunked};
use polars::prelude::*;

/// Extension methods for [`Series`]
pub trait SeriesExt {
    fn bound(&self) -> PolarsResult<&BoundChunked>;

    fn fatty_acid(&self) -> PolarsResult<&FattyAcidChunked>;
}

impl SeriesExt for Series {
    fn bound(&self) -> PolarsResult<&BoundChunked> {
        BoundChunked::new(self)
    }

    fn fatty_acid(&self) -> PolarsResult<&FattyAcidChunked> {
        FattyAcidChunked::new(self)
    }
}

pub mod fatty_acid;
pub mod triacylglycerol;
