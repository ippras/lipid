pub use self::expr::ExprExt;
use crate::polars::chunked_array::{BoundChunked, FattyAcidChunked};
use polars::prelude::*;
// use super::{FATTY_ACID_COLUMN, chunked_array::FattyAcidChunked, column::ColumnExt as _};

/// Fatty acid column name
pub const FATTY_ACID_COLUMN: &str = "FattyAcid";

/// Extension methods for [`Column`]
pub trait ColumnExt {
    fn fatty_acid(&self) -> PolarsResult<&FattyAcidChunked>;
}

impl ColumnExt for Column {
    fn fatty_acid(&self) -> PolarsResult<&FattyAcidChunked> {
        self.as_materialized_series().fatty_acid()
    }
}

/// Extension methods for [`DataFrame`]
pub trait DataFrameExt {
    fn fatty_acid(&self) -> PolarsResult<&FattyAcidChunked>;
}

impl DataFrameExt for DataFrame {
    fn fatty_acid(&self) -> PolarsResult<&FattyAcidChunked> {
        self[FATTY_ACID_COLUMN].fatty_acid()
    }
}

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

pub mod bound;
pub mod chunked_array;
pub mod expr;
