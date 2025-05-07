#![feature(debug_closure_helpers)]
#![feature(impl_trait_in_assoc_type)]
#![feature(mixed_integer_ops_unsigned_sub)]

pub mod bound;
pub mod kind;
pub mod triacylglycerol;

pub mod display;
#[cfg(feature = "polars")]
pub mod polars;
pub mod r#trait;

pub mod prelude {
    #[cfg(feature = "polars")]
    pub use crate::polars::{
        ColumnExt, DataFrameExt, SeriesExt,
        bound::{
            IDENTIFIER, IDENTIFIERS, INDEX, MAP,
            identifiers::{D, DC, DT, S, T, TC, TT, U, UC, UT},
        },
        chunked_array::{
            BOUND_DATA_TYPE, BoundChunked, FATTY_ACID_DATA_TYPE, FattyAcidChunked,
            FattyAcidListChunked, IdentifierIteratorExt as _, IndexIteratorExt as _,
        },
        expr::{
            ExprExt as _, FattyAcidExpr, TriacylglycerolExpr,
            triacylglycerol::permutation::Permutation as _,
        },
    };
    pub use crate::{
        bound::{Bound, Isomerism, Saturated, Type, Unsaturated, Unsaturation},
        r#const::*,
        display::{Elision, Options},
        kind::{Rco, Rcoo, Rcooch3, Rcooh},
        r#trait::{
            Atomic, EquivalentCarbonNumber, EquivalentChainLength, Kind, Mask, MaskExt, Mass,
        },
    };
}

pub mod r#const;
