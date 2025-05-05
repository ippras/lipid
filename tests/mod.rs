use lipid::prelude::*;
use polars::prelude::*;
use std::num::NonZeroI8;

fn fatty_acid<const N: usize>(
    fatty_acid: &[(Option<Option<NonZeroI8>>, Option<&str>); N],
) -> PolarsResult<FattyAcidChunked> {
    FattyAcidChunked::try_from(fatty_acid)
}

mod chunked_array;
mod display;
// mod expr;
