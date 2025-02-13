#![feature(decl_macro)]
#![feature(impl_trait_in_assoc_type)]

pub mod fatty_acid;
#[cfg(feature = "polars")]
pub mod polars;
pub mod triacylglycerol;

pub mod prelude {
    #[cfg(feature = "polars")]
    pub use crate::polars::{
        column::ColumnExt,
        data_frame::DataFrameExt,
        expr::{
            ExprExt as _,
            chain_length::{
                EquivalentCarbonNumber, EquivalentChainLengths, FractionalChainLength,
                Options as ChainLengthOptions,
            },
            fatty_acid::{r#const::*, filter::Filter, find::FindByName},
            mass::Mass as _,
            triacylglycerol::permutation::{Options as PermutationOptions, Permutation as _},
        },
        series::SeriesExt,
    };
}
