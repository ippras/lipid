use super::{FATTY_ACID_COLUMN, chunked_array::FattyAcidChunked, column::ColumnExt as _};
use polars::prelude::*;

/// Extension methods for [`DataFrame`]
pub trait DataFrameExt {
    fn fatty_acid(&self) -> PolarsResult<&FattyAcidChunked>;
}

impl DataFrameExt for DataFrame {
    fn fatty_acid(&self) -> PolarsResult<&FattyAcidChunked> {
        self[FATTY_ACID_COLUMN].fatty_acid()
    }
}
