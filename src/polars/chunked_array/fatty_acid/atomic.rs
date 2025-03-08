use super::FattyAcidChunked;
use polars::prelude::*;

impl FattyAcidChunked {
    /// [`BoundChunked::carbons`]
    pub fn carbons(&self) -> PolarsResult<UInt8Chunked> {
        self.map(|bounds| bounds.carbons()).collect()
    }

    /// [`BoundChunked::hydrogens`]
    pub fn hydrogens(&self) -> PolarsResult<UInt8Chunked> {
        self.map(|bounds| bounds.hydrogens()).collect()
    }
}
