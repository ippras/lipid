use crate::prelude::*;
use polars::prelude::*;
use std::{
    iter::{once, repeat_n},
    sync::LazyLock,
};

/// ω-3
pub static OMEGA3: LazyLock<Series> = LazyLock::new(|| omega(-3).unwrap());
/// ω-6
pub static OMEGA6: LazyLock<Series> = LazyLock::new(|| omega(-6).unwrap());
/// ω-9
pub static OMEGA9: LazyLock<Series> = LazyLock::new(|| omega(-9).unwrap());

fn omega(index: i8) -> PolarsResult<Series> {
    let s = index.abs() as usize - 1;
    Ok(df! {
        FattyAcidChunked::INDEX => Series::from_iter((index..0).map(Some).chain(once(None))),
        FattyAcidChunked::IDENTIFIER => Series::from_iter(once(Some(U)).chain(repeat_n(Some(S), s)).chain(once(None))).cast(&BOUND_DATA_TYPE)?,
    }?
    .into_struct(PlSmallStr::EMPTY).into_series())
}
