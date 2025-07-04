pub use self::{
    double_bound::{DoubleBoundChunked, DoubleBoundListChunked},
    fatty_acid::{FATTY_ACID_DATA_TYPE, FattyAcidChunked},
    fatty_acid_list::FattyAcidListChunked,
    formula::Formula,
    triple_bound::TripleBoundListChunked,
};
use polars::prelude::{enum_::EnumChunkedBuilder, *};

pub const DOUBLE_BOUNDS: &str = "DoubleBounds";
pub const FORMULA: &str = "Formula";
pub const INDEX: &str = "Index";
pub const PARITY: &str = "Parity";
pub const TRIPLE_BOUNDS: &str = "TripleBounds";

/// Identifier.
pub(crate) trait Identifier {
    fn append_to(self, builder: &mut EnumChunkedBuilder) -> PolarsResult<()>;
}

impl Identifier for &str {
    fn append_to(self, builder: &mut EnumChunkedBuilder) -> PolarsResult<()> {
        builder.append_str(self)?;
        Ok(())
    }
}

impl Identifier for Option<&str> {
    fn append_to(self, builder: &mut EnumChunkedBuilder) -> PolarsResult<()> {
        match self {
            Some(identifier) => builder.append_str(identifier)?,
            None => builder.append_null(),
        };
        Ok(())
    }
}

mod double_bound;
mod fatty_acid;
mod fatty_acid_list;
mod formula;
mod triple_bound;

// pub mod fatty_acid_new;
