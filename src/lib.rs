#![feature(decl_macro)]
#![feature(impl_trait_in_assoc_type)]

pub mod chain_length;
pub mod fatty_acid;
pub mod triacylglycerol;

pub mod prelude {
    pub use super::{
        chain_length::{
            EquivalentCarbonNumber, EquivalentChainLengths, FractionalChainLength,
            Options as ChainLengthOptions,
        },
        fatty_acid::polars::{
            column::ColumnExt,
            data_frame::DataFrameExt,
            expr::{ExprExt as _, r#const::*, filter::Filter, find::FindByName, mass::Mass as _},
            series::SeriesExt,
        },
        triacylglycerol::polars::expr::{
            ExprExt as _,
            mass::Mass as _,
            permutation::{Options as PermutationOptions, Permutation as _},
        },
    };
}
