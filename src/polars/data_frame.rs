use super::{FATTY_ACID_COLUMN, column::ColumnExt as _, series::FattyAcidSeries};
use polars::prelude::*;

/// Extension methods for [`DataFrame`]
pub trait DataFrameExt {
    fn fatty_acid(&self) -> FattyAcidSeries;

    fn fa(&self) -> FattyAcidSeries {
        self.fatty_acid()
    }
}

impl DataFrameExt for DataFrame {
    fn fatty_acid(&self) -> FattyAcidSeries {
        self[FATTY_ACID_COLUMN].fa()
    }
}
