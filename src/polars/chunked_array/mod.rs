pub use self::{
    bound::{BOUND_DATA_TYPE, BoundChunked},
    fatty_acid::{FATTY_ACID_DATA_TYPE, FattyAcidChunked, IdentifierIteratorExt, IndexIteratorExt},
    fatty_acid_list::FattyAcidListChunked,
};
use polars::prelude::{enum_::EnumChunkedBuilder, *};

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

mod bound;
mod fatty_acid;
mod fatty_acid_list;
mod unsaturated;
