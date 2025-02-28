use super::{chunked_array::fatty_acid::FattyAcidChunked, series::SeriesExt as _};
use polars::prelude::*;

/// Extension methods for [`Column`]
pub trait ColumnExt {
    fn fatty_acid(&self) -> PolarsResult<FattyAcidChunked>;
}

impl ColumnExt for Column {
    fn fatty_acid(&self) -> PolarsResult<FattyAcidChunked> {
        self.as_materialized_series().fatty_acid()
    }
}
