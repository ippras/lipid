use super::Identifier;
use crate::prelude::{physical::S, *};
use polars::prelude::{enum_::EnumChunkedBuilder, *};
use std::{
    borrow::Cow,
    fmt::{self, Debug, Display, Formatter, Write, from_fn},
    num::NonZeroI8,
    sync::LazyLock,
};

/// Fatty acid chunked array.
#[derive(Clone, Debug)]
pub struct FattyAcidListChunked {
    pub carbons: UInt8Chunked,
    pub unsaturated: UnsaturatedListChunked,
}

/// Unsaturated chunked array.
#[derive(Clone, Debug)]
pub struct UnsaturatedListChunked(ListChunked);

/// Unsaturated chunked array.
#[derive(Clone, Debug)]
pub struct UnsaturatedStructChunked {
    pub index: Int8Chunked,
    pub parity: BooleanChunked,
    pub unsaturation: UInt8Chunked,
}
