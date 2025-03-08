use super::FattyAcidChunked;
use polars::prelude::*;

impl FattyAcidChunked {
    #[inline]
    pub fn equivalent_carbon_number(&self) -> PolarsResult<UInt8Chunked> {
        self.map(|bounds| bounds.equivalent_carbon_number())
            .collect()
    }
}
