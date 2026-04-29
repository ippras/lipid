#![feature(array_try_map)]
#![feature(custom_inner_attributes)]
#![feature(impl_trait_in_assoc_type)]

use crate::prelude::*;
use polars::prelude::*;

/// Extension methods for [`DataFrame`]
pub trait DataFrameExt {
    fn fatty_acid(&self) -> &FattyAcidChunked {
        self.try_fatty_acid().unwrap()
    }

    fn try_fatty_acid(&self) -> PolarsResult<&FattyAcidChunked>;
}

impl DataFrameExt for DataFrame {
    fn try_fatty_acid(&self) -> PolarsResult<&FattyAcidChunked> {
        self[FATTY_ACID].try_fatty_acid()
    }
}

/// Extension methods for [`Column`]
pub trait ColumnExt {
    fn fatty_acid(&self) -> &FattyAcidChunked {
        self.try_fatty_acid().unwrap()
    }

    fn triacylglycerol(&self) -> &TriacylglycerolChunked {
        self.try_triacylglycerol().unwrap()
    }

    fn try_fatty_acid(&self) -> PolarsResult<&FattyAcidChunked>;

    fn try_triacylglycerol(&self) -> PolarsResult<&TriacylglycerolChunked>;
}

impl ColumnExt for Column {
    fn try_fatty_acid(&self) -> PolarsResult<&FattyAcidChunked> {
        self.as_materialized_series().try_fatty_acid()
    }

    fn try_triacylglycerol(&self) -> PolarsResult<&TriacylglycerolChunked> {
        self.as_materialized_series().try_triacylglycerol()
    }
}

/// Extension methods for [`Series`]
pub trait SeriesExt {
    fn fatty_acid(&self) -> &FattyAcidChunked {
        self.try_fatty_acid().unwrap()
    }

    fn triacylglycerol(&self) -> &TriacylglycerolChunked {
        self.try_triacylglycerol().unwrap()
    }

    fn try_fatty_acid(&self) -> PolarsResult<&FattyAcidChunked>;

    fn try_triacylglycerol(&self) -> PolarsResult<&TriacylglycerolChunked>;
}

impl SeriesExt for Series {
    fn try_fatty_acid(&self) -> PolarsResult<&FattyAcidChunked> {
        self.try_into()
    }

    fn try_triacylglycerol(&self) -> PolarsResult<&TriacylglycerolChunked> {
        self.try_into()
    }
}

pub mod prelude {
    #[cfg(feature = "biodiesel")]
    pub use crate::expr::fatty_acid::properties::biodiesel::BiodieselProperties;
    pub use crate::{
        ColumnExt, DataFrameExt, SeriesExt,
        chunked_array::{
            fatty_acid::FattyAcidChunked, triacylglycerol::TriacylglycerolChunked,
            unsaturated::UnsaturatedChunked,
        },
        r#const::*,
        data_type,
        expr::{
            ExprExt as _, FattyAcidExpr, TriacylglycerolExpr,
            fatty_acid::{CARBON, FATTY_ACID, INDEX, INDICES, PARITY, TRIPLE},
            triacylglycerol::{
                LABEL, STEREOSPECIFIC_NUMBERS, STEREOSPECIFIC_NUMBERS1, STEREOSPECIFIC_NUMBERS2,
                STEREOSPECIFIC_NUMBERS3, STEREOSPECIFIC_NUMBERS12_23, STEREOSPECIFIC_NUMBERS13,
                STEREOSPECIFIC_NUMBERS123, TRIACYLGLYCEROL, permutation::Permutation as _,
            },
        },
        field,
        kind::{Rco, Rcoo, Rcooch3, Rcooh},
        r#struct::{
            fatty_acid::{FattyAcid, Unsaturated},
            triacylglycerol::{Stereospecificity, Triacylglycerol},
        },
        r#trait::{
            Atomic, EquivalentCarbonNumber, EquivalentChainLength, Kind, RelativeAtomicMass,
            fatty_acid::{
                FattyAcidMaskByBounds, FattyAcidMaskByDoubleBounds, FattyAcidMaskByParity,
                FattyAcidSumByBounds, FattyAcidSumByDoubleBounds, FattyAcidTrait,
            },
        },
    };
    pub use fatty_acid_macro::fatty_acid;
}

pub mod chunked_array;
pub mod r#const;
pub mod expr;
pub mod kind;
pub mod r#macro;
pub mod r#struct;
pub mod r#trait;
