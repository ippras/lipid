use lipid::prelude::*;
use polars::prelude::*;
use std::{num::NonZeroI8, sync::LazyLock};

/// [byrdwell.com](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static FATTY_ACIDS: LazyLock<DataFrame> = LazyLock::new(|| {
    fatty_acids()
        .unwrap()
        .with_row_index(PlSmallStr::from_static("Index"), None)
        .unwrap()
});

// fn fa<const N: usize>(fatty_acid: FattyAcid<N>) -> PolarsResult<DataFrame> {
//     df! {
//         "FattyAcid" => [
//             Some(Series::from_iter(fatty_acid).cast(&IDENTIFIER_DATA_TYPE)?),
//         ],
//     }
// }

fn fatty_acids() -> PolarsResult<DataFrame> {
    df! {
        "FattyAcid" => [
            Some(FattyAcidChunked::try_from(&C4)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 0
            Some(FattyAcidChunked::try_from(&C5)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 1
            Some(FattyAcidChunked::try_from(&C6)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 2
            Some(FattyAcidChunked::try_from(&C7)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 3
            Some(FattyAcidChunked::try_from(&C8)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 4
            Some(FattyAcidChunked::try_from(&C9)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 5
            Some(FattyAcidChunked::try_from(&C10)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 6
            Some(FattyAcidChunked::try_from(&C11)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 7
            Some(FattyAcidChunked::try_from(&C12)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 8
            Some(FattyAcidChunked::try_from(&C13)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 9
            Some(FattyAcidChunked::try_from(&C14)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 10
            Some(FattyAcidChunked::try_from(&C15)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 11
            Some(FattyAcidChunked::try_from(&C16)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 12
            Some(FattyAcidChunked::try_from(&C16DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 13
            Some(FattyAcidChunked::try_from(&C16DT9)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 14
            Some(FattyAcidChunked::try_from(&C17)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 15
            Some(FattyAcidChunked::try_from(&C18)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 16
            Some(FattyAcidChunked::try_from(&C18DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 17
            Some(FattyAcidChunked::try_from(&C18DT9)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 18
            Some(FattyAcidChunked::try_from(&C18DC9DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 19
            Some(FattyAcidChunked::try_from(&C18DC6DC9DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 20
            Some(FattyAcidChunked::try_from(&C18DC8DT10DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 21
            Some(FattyAcidChunked::try_from(&C18DC9DC12DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 22
            Some(FattyAcidChunked::try_from(&C18DC9DT11DT13)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 23
            Some(FattyAcidChunked::try_from(&C18DT9DT11DC13)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 24
            Some(FattyAcidChunked::try_from(&C18DT9DT11DT13)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 25
            Some(FattyAcidChunked::try_from(&C18DC6DC9DC12DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 26
            Some(FattyAcidChunked::try_from(&C19)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 27
            Some(FattyAcidChunked::try_from(&C20)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 28
            Some(FattyAcidChunked::try_from(&C20DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 29
            Some(FattyAcidChunked::try_from(&C20DC11)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 30
            Some(FattyAcidChunked::try_from(&C20DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 31
            Some(FattyAcidChunked::try_from(&C20DC5DC8DC11)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 32
            Some(FattyAcidChunked::try_from(&C20DC8DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 33
            Some(FattyAcidChunked::try_from(&C20DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 34
            Some(FattyAcidChunked::try_from(&C20DC5DC8DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 35
            Some(FattyAcidChunked::try_from(&C20DC8DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 36
            Some(FattyAcidChunked::try_from(&C20DC5DC8DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 37
            Some(FattyAcidChunked::try_from(&C21)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 38
            Some(FattyAcidChunked::try_from(&C22)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 39
            Some(FattyAcidChunked::try_from(&C22DC13)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 40
            Some(FattyAcidChunked::try_from(&C22DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 41
            Some(FattyAcidChunked::try_from(&C22DC5DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 42
            Some(FattyAcidChunked::try_from(&C22DC7DC10DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 43
            Some(FattyAcidChunked::try_from(&C22DC7DC10DC13DC16DC19)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 44
            Some(FattyAcidChunked::try_from(&C22DC4DC7DC10DC13DC16DC19)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 45
            Some(FattyAcidChunked::try_from(&C23)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 46
            Some(FattyAcidChunked::try_from(&C24)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 47
            Some(FattyAcidChunked::try_from(&C24DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 48
            Some(FattyAcidChunked::try_from(&C24DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 49
            Some(FattyAcidChunked::try_from(&C24DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 50
            Some(FattyAcidChunked::try_from(&C24DC9DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 51
            Some(FattyAcidChunked::try_from(&C24DC6DC9DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 52
            Some(FattyAcidChunked::try_from(&C24DC6DC9DC12DC15DC18DC21)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 53
            Some(FattyAcidChunked::try_from(&C25)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 54
            Some(FattyAcidChunked::try_from(&C26)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 55
            Some(FattyAcidChunked::try_from(&C26DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 56
            Some(FattyAcidChunked::try_from(&C27)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 57
            Some(FattyAcidChunked::try_from(&C28)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 58
            Some(FattyAcidChunked::try_from(&C29)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 59
            Some(FattyAcidChunked::try_from(&C30)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 60
            Some(FattyAcidChunked::try_from(&C30DC21)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 61
            Some(FattyAcidChunked::try_from(&C31)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 62
            Some(FattyAcidChunked::try_from(&C32)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 63
            Some(FattyAcidChunked::try_from(&C33)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 64
            Some(FattyAcidChunked::try_from(&C34)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 65
            Some(FattyAcidChunked::try_from(&C35)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 66
            Some(FattyAcidChunked::try_from(&C36)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 67
            None, // 68
        ],
        "Value" => [
            Some(4.0),
            Some(5.0),
            Some(6.0),
            Some(7.0),
            Some(8.0),
            Some(9.0),
            Some(10.0),
            Some(11.0),
            Some(12.0),
            Some(13.0),
            Some(14.0),
            Some(15.0),
            Some(16.0),
            Some(14.0),
            Some(14.0),
            Some(17.0),
            Some(18.0),
            Some(16.0),
            Some(16.0),
            Some(14.0),
            Some(12.0),
            Some(12.0),
            Some(12.0),
            Some(12.0),
            Some(12.0),
            Some(12.0),
            Some(10.0),
            Some(19.0),
            Some(20.0),
            Some(18.0),
            Some(18.0),
            Some(16.0),
            Some(14.0),
            Some(14.0),
            Some(14.0),
            Some(12.0),
            Some(12.0),
            Some(10.0),
            Some(21.0),
            Some(22.0),
            Some(20.0),
            Some(18.0),
            Some(16.0),
            Some(14.0),
            Some(12.0),
            Some(10.0),
            Some(23.0),
            Some(24.0),
            Some(22.0),
            Some(20.0),
            Some(18.0),
            Some(16.0),
            Some(14.0),
            Some(12.0),
            Some(25.0),
            Some(26.0),
            Some(24.0),
            Some(27.0),
            Some(28.0),
            Some(29.0),
            Some(30.0),
            Some(28.0),
            Some(31.0),
            Some(32.0),
            Some(33.0),
            Some(34.0),
            Some(35.0),
            Some(36.0),
            None,
        ],
    }
}

fn fatty_acid1<const N: usize>(
    fatty_acid: &[(Option<Option<NonZeroI8>>, Option<&'static str>); N],
) -> PolarsResult<DataFrame> {
    df! {
        "FattyAcid" => [
            FattyAcidChunked::try_from(fatty_acid)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
        ],
    }
    // FattyAcidChunked::try_from(&fatty_acid)
}

fn epsilon(received: f64, expected: f64) -> bool {
    (received - expected).abs() < f64::EPSILON
}

mod chain_length;
// mod equal;
// mod factors;
#[cfg(feature = "indices")]
mod indices;
#[cfg(feature = "map")]
mod map;
#[cfg(feature = "mask")]
mod mask;
#[cfg(feature = "mass")]
mod mass;
// #[cfg(feature = "select")]
// mod select;
