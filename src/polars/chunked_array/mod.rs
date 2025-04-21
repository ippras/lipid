pub use self::{
    bound::{BOUND_DATA_TYPE, BoundChunked},
    fatty_acid::{FATTY_ACID_DATA_TYPE, FattyAcidChunked, IdentifierIteratorExt, IndexIteratorExt},
    fatty_acid_list::FattyAcidListChunked,
};

mod bound;
mod fatty_acid;
mod fatty_acid_list;
