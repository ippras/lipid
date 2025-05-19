// pub use self::expr::ExprExt;
use crate::prelude::{FattyAcidChunked, FattyAcidListChunked};
use polars::prelude::*;

/// Fatty acid column name
pub const FATTY_ACID: &str = "FattyAcid";

/// Extension methods for [`DataFrame`]
pub trait DataFrameExt {
    fn fatty_acid_list(&self) -> &FattyAcidListChunked {
        self.try_fatty_acid_list().unwrap()
    }

    fn try_fatty_acid_list(&self) -> PolarsResult<&FattyAcidListChunked>;
}

impl DataFrameExt for DataFrame {
    fn try_fatty_acid_list(&self) -> PolarsResult<&FattyAcidListChunked> {
        self[FATTY_ACID].try_fatty_acid_list()
    }
}

/// Extension methods for [`Column`]
pub trait ColumnExt {
    fn fatty_acid_list(&self) -> &FattyAcidListChunked {
        self.try_fatty_acid_list().unwrap()
    }

    fn try_fatty_acid_list(&self) -> PolarsResult<&FattyAcidListChunked>;
}

impl ColumnExt for Column {
    fn try_fatty_acid_list(&self) -> PolarsResult<&FattyAcidListChunked> {
        self.as_materialized_series().try_fatty_acid_list()
    }
}

/// Extension methods for [`Series`]
pub trait SeriesExt {
    fn fatty_acid_list(&self) -> &FattyAcidListChunked {
        self.try_fatty_acid_list().unwrap()
    }

    fn fatty_acid(&self) -> FattyAcidChunked {
        self.try_fatty_acid().unwrap()
    }

    fn try_fatty_acid_list(&self) -> PolarsResult<&FattyAcidListChunked>;

    fn try_fatty_acid(&self) -> PolarsResult<FattyAcidChunked>;
}

impl SeriesExt for Series {
    fn try_fatty_acid_list(&self) -> PolarsResult<&FattyAcidListChunked> {
        self.try_into()
    }

    fn try_fatty_acid(&self) -> PolarsResult<FattyAcidChunked> {
        self.try_into()
    }
}

pub mod bound;
pub mod chunked_array;
pub mod expr;
