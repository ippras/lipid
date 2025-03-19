use lipid::prelude::*;
use polars::prelude::*;
use std::sync::LazyLock;

/// [byrdwell.com](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static FATTY_ACIDS: LazyLock<DataFrame> = LazyLock::new(|| {
    fatty_acids()
        .unwrap()
        .with_row_index("Index".into(), None)
        .unwrap()
});

fn fatty_acids() -> PolarsResult<DataFrame> {
    df! {
        "FattyAcid" => [
            Some(Series::from_iter(C4U0).cast(&BOUND_DATA_TYPE).unwrap()), // 0
            Some(Series::from_iter(C5U0).cast(&BOUND_DATA_TYPE).unwrap()), // 1
            Some(Series::from_iter(C6U0).cast(&BOUND_DATA_TYPE).unwrap()), // 2
            Some(Series::from_iter(C7U0).cast(&BOUND_DATA_TYPE).unwrap()), // 3
            Some(Series::from_iter(C8U0).cast(&BOUND_DATA_TYPE).unwrap()), // 4
            Some(Series::from_iter(C9U0).cast(&BOUND_DATA_TYPE).unwrap()), // 5
            Some(Series::from_iter(C10U0).cast(&BOUND_DATA_TYPE).unwrap()), // 6
            Some(Series::from_iter(C11U0).cast(&BOUND_DATA_TYPE).unwrap()), // 7
            Some(Series::from_iter(C12U0).cast(&BOUND_DATA_TYPE).unwrap()), // 8
            Some(Series::from_iter(C13U0).cast(&BOUND_DATA_TYPE).unwrap()), // 9
            Some(Series::from_iter(C14U0).cast(&BOUND_DATA_TYPE).unwrap()), // 10
            Some(Series::from_iter(C15U0).cast(&BOUND_DATA_TYPE).unwrap()), // 11
            Some(Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE).unwrap()), // 12
            Some(Series::from_iter(C16U1DC9).cast(&BOUND_DATA_TYPE).unwrap()), // 13
            Some(Series::from_iter(C16U1DT9).cast(&BOUND_DATA_TYPE).unwrap()), // 14
            Some(Series::from_iter(C17U0).cast(&BOUND_DATA_TYPE).unwrap()), // 15
            Some(Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE).unwrap()), // 16
            Some(Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE).unwrap()), // 17
            Some(Series::from_iter(C18U1DT9).cast(&BOUND_DATA_TYPE).unwrap()), // 18
            Some(Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE).unwrap()), // 19
            Some(Series::from_iter(C18U3DC6DC9DC12).cast(&BOUND_DATA_TYPE).unwrap()), // 20
            Some(Series::from_iter(C18U3DC8DT10DC12).cast(&BOUND_DATA_TYPE).unwrap()), // 21
            Some(Series::from_iter(C18U3DC9DC12DC15).cast(&BOUND_DATA_TYPE).unwrap()), // 22
            Some(Series::from_iter(C18U3DC9DT11DT13).cast(&BOUND_DATA_TYPE).unwrap()), // 23
            Some(Series::from_iter(C18U3DT9DT11DC13).cast(&BOUND_DATA_TYPE).unwrap()), // 24
            Some(Series::from_iter(C18U3DT9DT11DT13).cast(&BOUND_DATA_TYPE).unwrap()), // 25
            Some(Series::from_iter(C18U4DC6DC9DC12DC15).cast(&BOUND_DATA_TYPE).unwrap()), // 26
            Some(Series::from_iter(C19U0).cast(&BOUND_DATA_TYPE).unwrap()), // 27
            Some(Series::from_iter(C20U0).cast(&BOUND_DATA_TYPE).unwrap()), // 28
            Some(Series::from_iter(C20U1DC9).cast(&BOUND_DATA_TYPE).unwrap()), // 29
            Some(Series::from_iter(C20U1DC11).cast(&BOUND_DATA_TYPE).unwrap()), // 30
            Some(Series::from_iter(C20U2DC11DC14).cast(&BOUND_DATA_TYPE).unwrap()), // 31
            Some(Series::from_iter(C20U3DC5DC8DC11).cast(&BOUND_DATA_TYPE).unwrap()), // 32
            Some(Series::from_iter(C20U3DC8DC11DC14).cast(&BOUND_DATA_TYPE).unwrap()), // 33
            Some(Series::from_iter(C20U3DC11DC14DC17).cast(&BOUND_DATA_TYPE).unwrap()), // 34
            Some(Series::from_iter(C20U4DC5DC8DC11DC14).cast(&BOUND_DATA_TYPE).unwrap()), // 35
            Some(Series::from_iter(C20U4DC8DC11DC14DC17).cast(&BOUND_DATA_TYPE).unwrap()), // 36
            Some(Series::from_iter(C20U5DC5DC8DC11DC14DC17).cast(&BOUND_DATA_TYPE).unwrap()), // 37
            Some(Series::from_iter(C21U0).cast(&BOUND_DATA_TYPE).unwrap()), // 38
            Some(Series::from_iter(C22U0).cast(&BOUND_DATA_TYPE).unwrap()), // 39
            Some(Series::from_iter(C22U1DC13).cast(&BOUND_DATA_TYPE).unwrap()), // 40
            Some(Series::from_iter(C22U2DC13DC16).cast(&BOUND_DATA_TYPE).unwrap()), // 41
            Some(Series::from_iter(C22U3DC5DC13DC16).cast(&BOUND_DATA_TYPE).unwrap()), // 42
            Some(Series::from_iter(C22U4DC7DC10DC13DC16).cast(&BOUND_DATA_TYPE).unwrap()), // 43
            Some(Series::from_iter(C22U5DC7DC10DC13DC16DC19).cast(&BOUND_DATA_TYPE).unwrap()), // 44
            Some(Series::from_iter(C22U6DC4DC7DC10DC13DC16DC19).cast(&BOUND_DATA_TYPE).unwrap()), // 45
            Some(Series::from_iter(C23U0).cast(&BOUND_DATA_TYPE).unwrap()), // 46
            Some(Series::from_iter(C24U0).cast(&BOUND_DATA_TYPE).unwrap()), // 47
            Some(Series::from_iter(C24U1DC15).cast(&BOUND_DATA_TYPE).unwrap()), // 48
            Some(Series::from_iter(C24U2DC15DC18).cast(&BOUND_DATA_TYPE).unwrap()), // 49
            Some(Series::from_iter(C24U3DC12DC15DC18).cast(&BOUND_DATA_TYPE).unwrap()), // 50
            Some(Series::from_iter(C24U4DC9DC12DC15DC18).cast(&BOUND_DATA_TYPE).unwrap()), // 51
            Some(Series::from_iter(C24U5DC6DC9DC12DC15DC18).cast(&BOUND_DATA_TYPE).unwrap()), // 52
            Some(Series::from_iter(C24U6DC6DC9DC12DC15DC18DC21).cast(&BOUND_DATA_TYPE).unwrap()), // 53
            Some(Series::from_iter(C25U0).cast(&BOUND_DATA_TYPE).unwrap()), // 54
            Some(Series::from_iter(C26U0).cast(&BOUND_DATA_TYPE).unwrap()), // 55
            Some(Series::from_iter(C26U1DC17).cast(&BOUND_DATA_TYPE).unwrap()), // 56
            Some(Series::from_iter(C27U0).cast(&BOUND_DATA_TYPE).unwrap()), // 57
            Some(Series::from_iter(C28U0).cast(&BOUND_DATA_TYPE).unwrap()), // 58
            Some(Series::from_iter(C29U0).cast(&BOUND_DATA_TYPE).unwrap()), // 59
            Some(Series::from_iter(C30U0).cast(&BOUND_DATA_TYPE).unwrap()), // 60
            Some(Series::from_iter(C30U1DC21).cast(&BOUND_DATA_TYPE).unwrap()), // 61
            Some(Series::from_iter(C31U0).cast(&BOUND_DATA_TYPE).unwrap()), // 62
            Some(Series::from_iter(C32U0).cast(&BOUND_DATA_TYPE).unwrap()), // 63
            Some(Series::from_iter(C33U0).cast(&BOUND_DATA_TYPE).unwrap()), // 64
            Some(Series::from_iter(C34U0).cast(&BOUND_DATA_TYPE).unwrap()), // 65
            Some(Series::from_iter(C35U0).cast(&BOUND_DATA_TYPE).unwrap()), // 66
            Some(Series::from_iter(C36U0).cast(&BOUND_DATA_TYPE).unwrap()), // 67
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

fn epsilon(received: f64, expected: f64) -> bool {
    (received - expected).abs() < f64::EPSILON
}

mod chain_length;
mod equal;
mod factors;
#[cfg(feature = "indices")]
mod indices;
#[cfg(feature = "map")]
mod map;
#[cfg(feature = "mask")]
mod mask;
#[cfg(feature = "mass")]
mod mass;
#[cfg(feature = "select")]
mod select;
