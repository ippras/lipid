use super::series::{FattyAcidSeries, SeriesExt as _};
use polars::prelude::*;

/// Extension methods for [`Column`]
pub trait ColumnExt {
    fn fatty_acid(&self) -> FattyAcidSeries;

    fn fa(&self) -> FattyAcidSeries {
        self.fatty_acid()
    }
}

impl ColumnExt for Column {
    fn fatty_acid(&self) -> FattyAcidSeries {
        self.as_materialized_series().fa()
    }
}
