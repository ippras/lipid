#![feature(decl_macro)]
#![feature(impl_trait_in_assoc_type)]

pub mod fatty_acid;
pub mod triacylglycerol;

#[cfg(feature = "polars")]
pub mod polars;

pub mod prelude {
    pub use crate::fatty_acid::FattyAcid;
    #[cfg(feature = "polars")]
    pub use crate::polars::{
        bound::{Bound, Isomerism, Type, Unaturation, Unsaturated},
        column::ColumnExt,
        data_frame::DataFrameExt,
        expr::{
            ExprExt as _,
            chain_length::{
                EquivalentCarbonNumber, EquivalentChainLengths, FractionalChainLength,
                Options as ChainLengthOptions,
            },
            fatty_acid::{
                r#const::*,
                factor::{Selectivity, ef},
                filter::Filter,
                find::FindByName,
            },
            mass::Mass as _,
            triacylglycerol::permutation::{Options as PermutationOptions, Permutation as _},
        },
        fatty_acid::{Kind as FattyAcidKind, Rco, Rcoo, Rcooch3, Rcooh},
        series::{
            SeriesExt,
            bound::{
                BoundSeries,
                display::{Elision, Kind as DisplayKind, Options},
                is_not_saturated, is_not_unsaturated, is_saturated, is_unsaturated,
            },
            fatty_acid::{bounds, carbons, filter, unsaturated_indexed},
        },
    };
}
