#![feature(impl_trait_in_assoc_type)]

pub use self::fatty_acid::FattyAcid;

pub mod bound;
pub mod fatty_acid;
pub mod triacylglycerol;

pub mod display;
#[cfg(feature = "polars")]
pub mod polars;

pub mod prelude {
    #[cfg(feature = "polars")]
    pub use crate::polars::{
        bound::{
            BOUND_DATA_TYPE,
            identifiers::{D, DC, DT, S, T, TC, TT, U, UC, UT},
        },
        chunked_array::{BoundChunked, FattyAcidChunked},
        column::ColumnExt,
        data_frame::DataFrameExt,
        expr::{
            ExprExt as _, FattyAcidExpr,
            triacylglycerol::permutation::{Options as PermutationOptions, Permutation as _},
        },
        series::SeriesExt,
    };
    pub use crate::{
        bound::{Bound, Isomerism, Type, Unsaturated, Unsaturation},
        r#const::*,
        display::{
            Elision,
            Kind::{Delta, System},
            Options,
        },
        fatty_acid::{FattyAcid, Kind, Rco, Rcoo, Rcooch3, Rcooh},
    };
}

pub mod r#const;
