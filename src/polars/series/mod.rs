use crate::polars::chunked_array::fatty_acid::FattyAcidChunked;
use polars::prelude::*;

/// Extension methods for [`Series`]
pub trait SeriesExt {
    fn fatty_acid(&self) -> PolarsResult<FattyAcidChunked>;
}

impl SeriesExt for Series {
    fn fatty_acid(&self) -> PolarsResult<FattyAcidChunked> {
        FattyAcidChunked::new(self)
    }
}

pub mod bound;
pub mod fatty_acid;
