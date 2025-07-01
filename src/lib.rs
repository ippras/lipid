#![feature(custom_inner_attributes)]
#![feature(debug_closure_helpers)]
#![feature(impl_trait_in_assoc_type)]
#![feature(exact_size_is_empty)]
// #![feature(mixed_integer_ops_unsigned_sub)]

use crate::prelude::*;
use polars::prelude::*;

/// Extension methods for [`DataFrame`]
pub trait DataFrameExt {
    fn fatty_acid_list(&self) -> &FattyAcidChunked {
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

    fn try_fatty_acid(&self) -> PolarsResult<&FattyAcidChunked>;
}

impl ColumnExt for Column {
    fn try_fatty_acid(&self) -> PolarsResult<&FattyAcidChunked> {
        self.as_materialized_series().try_fatty_acid()
    }
}

/// Extension methods for [`Series`]
pub trait SeriesExt {
    fn fatty_acid(&self) -> &FattyAcidChunked {
        self.try_fatty_acid().unwrap()
    }

    fn try_fatty_acid(&self) -> PolarsResult<&FattyAcidChunked>;
}

impl SeriesExt for Series {
    fn try_fatty_acid(&self) -> PolarsResult<&FattyAcidChunked> {
        self.try_into()
    }
}

pub mod prelude {
    pub use crate::{
        ColumnExt, DataFrameExt, SeriesExt,
        bound::{Bound, Explicit, Isomerism, Saturated, Type, Unsaturated, Unsaturation},
        chunked_array::FattyAcidChunked,
        r#const::*,
        data_type,
        display::{Elision, Options},
        expr::{
            ExprExt as _, FattyAcidExpr, TriacylglycerolExpr,
            fatty_acid::{BOUNDS, CARBON, FATTY_ACID, INDEX, PARITY, TRIPLE},
            triacylglycerol::{
                LABEL, STEREOSPECIFIC_NUMBER1, STEREOSPECIFIC_NUMBER2, STEREOSPECIFIC_NUMBER3,
                TRIACYLGLYCEROL, permutation::Permutation as _,
            },
        },
        field,
        kind::{Rco, Rcoo, Rcooch3, Rcooh},
        r#trait::{
            Atomic, EquivalentCarbonNumber, EquivalentChainLength, IdentifierMask, Kind, MaskExt,
            Mass,
        },
    };
}

pub mod bound;
pub mod chunked_array;
pub mod r#const;
pub mod display;
pub mod expr;
pub mod kind;
pub mod r#macro;
pub mod r#trait;
