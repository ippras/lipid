#![feature(custom_inner_attributes)]
#![feature(debug_closure_helpers)]
#![feature(impl_trait_in_assoc_type)]
#![feature(exact_size_is_empty)]
// #![feature(mixed_integer_ops_unsigned_sub)]

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
        ColumnExt,
        DataFrameExt,
        FATTY_ACID,
        SeriesExt,
        // bound::{
        //     BOUND, IDENTIFIERS, INDEX, MAP,
        //     identifiers::{
        //         logical::{B, D, DC, DT, S, T, TC, TT, U, UC, UT},
        //         physical,
        //     },
        // },
        chunked_array::{
            DOUBLE_BOUNDS, FATTY_ACID_DATA_TYPE, FORMULA, FattyAcidChunked, FattyAcidListChunked,
            Formula, INDEX, PARITY, TRIPLE_BOUNDS, TripleBoundListChunked,
        },
        // expr::{
        //     ExprExt as _, FattyAcidExpr, TriacylglycerolExpr,
        //     triacylglycerol::permutation::Permutation as _,
        // },
    };
    pub use crate::{
        bound::{Bound, Explicit, Isomerism, Saturated, Type, Unsaturated, Unsaturation},
        // r#const::*,
        display::{Elision, Options},
        kind::{Rco, Rcoo, Rcooch3, Rcooh},
        r#trait::{
            Atomic, EquivalentCarbonNumber, EquivalentChainLength, IdentifierMask, Kind, MaskExt,
            Mass,
        },
    };
}

// pub mod r#const;
