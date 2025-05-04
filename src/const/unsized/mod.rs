use crate::prelude::*;
use polars::prelude::*;
use std::{iter::repeat_n, sync::LazyLock};

pub static C26U2: LazyLock<Series> = LazyLock::new(|| cu(26, 2).unwrap());
pub static C26U3: LazyLock<Series> = LazyLock::new(|| cu(26, 3).unwrap());
pub static C26U4: LazyLock<Series> = LazyLock::new(|| cu(26, 4).unwrap());
pub static C26U5: LazyLock<Series> = LazyLock::new(|| cu(26, 5).unwrap());
pub static C26U6: LazyLock<Series> = LazyLock::new(|| cu(26, 6).unwrap());
pub static C28U1: LazyLock<Series> = LazyLock::new(|| cu(28, 1).unwrap());
pub static C28U2: LazyLock<Series> = LazyLock::new(|| cu(28, 2).unwrap());
pub static C32U1: LazyLock<Series> = LazyLock::new(|| cu(32, 1).unwrap());
pub static C32U2: LazyLock<Series> = LazyLock::new(|| cu(32, 2).unwrap());
pub static C34U1: LazyLock<Series> = LazyLock::new(|| cu(34, 1).unwrap());
pub static C34U2: LazyLock<Series> = LazyLock::new(|| cu(34, 2).unwrap());
pub static C36U1: LazyLock<Series> = LazyLock::new(|| cu(36, 1).unwrap());
pub static C36U2: LazyLock<Series> = LazyLock::new(|| cu(36, 2).unwrap());

fn cu(c: u8, u: u8) -> PolarsResult<Series> {
    let b = c as usize - 1;
    let u = u as usize;
    let s = b - u;
    Ok(df! {
        FattyAcidChunked::INDEX => Series::from_iter(repeat_n(0i8, b)),
        FattyAcidChunked::IDENTIFIER => Series::from_iter(repeat_n(U, u).chain(repeat_n(S, s))).cast(&BOUND_DATA_TYPE)?,
    }?
    .into_struct(PlSmallStr::EMPTY).into_series())
}

pub mod omega;
