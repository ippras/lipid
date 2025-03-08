#![feature(impl_trait_in_assoc_type)]

pub mod fatty_acid;
pub mod triacylglycerol;

pub mod display;
#[cfg(feature = "polars")]
pub mod polars;

pub mod prelude {
    #[cfg(feature = "polars")]
    pub use crate::polars::{
        bound::{
            BOUND_DATA_TYPE, Bound, Isomerism, Type, Unsaturated, Unsaturation,
            identifiers::{D, DC, DT, S, T, TC, TT, U, UC, UT},
        },
        chunked_array::{BoundChunked, FattyAcidChunked},
        column::ColumnExt,
        data_frame::DataFrameExt,
        expr::{
            ExprExt as _,
            triacylglycerol::permutation::{Options as PermutationOptions, Permutation as _},
        },
        series::{
            SeriesExt,
            fatty_acid::{mask, selectivity_factor, unsaturated_indexed},
        },
    };
    pub use crate::{
        display::{
            Elision,
            Kind::{Delta, System},
            Options,
        },
        fatty_acid::{Kind, Rco, Rcoo, Rcooch3, Rcooh},
    };
    pub use fatty_acid_macro::fatty_acid;
}
