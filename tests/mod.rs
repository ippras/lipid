//! [byrdwell.com](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)

#![feature(custom_inner_attributes)]

use fatty_acid_macro::fatty_acid;
use lipid::prelude::*;
use polars::prelude::*;
use std::{num::NonZero, sync::LazyLock};

const DECIMALS: usize = 4;

macro_rules! assert_epsilon {
    ($got:ident, $expected:expr) => {
        assert!(($got - $expected).abs() < f64::EPSILON);
    };
}

macro_rules! index {
    (C4) => {
        0
    };
    (C5) => {
        1
    };
    (C6) => {
        2
    };
    (C7) => {
        3
    };
    (C8) => {
        4
    };
    (C9) => {
        5
    };
    (C10) => {
        6
    };
    (C11) => {
        7
    };
    (C12) => {
        8
    };
    (C13) => {
        9
    };
    (C14) => {
        10
    };
    (C15) => {
        11
    };
    (C16) => {
        12
    };
    (C16DC9) => {
        13
    };
    (C16DT9) => {
        14
    };
    (C17) => {
        15
    };
    (C18) => {
        16
    };
    (C18DC9) => {
        17
    };
    (C18DT9) => {
        18
    };
    (C18DC9DC12) => {
        19
    };
    (C18DC6DC9DC12) => {
        20
    };
    (C18DC8DT10DC12) => {
        21
    };
    (C18DC9DC12DC15) => {
        22
    };
    (C18DC9DT11DT13) => {
        23
    };
    (C18DT9DT11DC13) => {
        24
    };
    (C18DT9DT11DT13) => {
        25
    };
    (C18DC6DC9DC12DC15) => {
        26
    };
    (C19) => {
        27
    };
    (C20) => {
        28
    };
    (C20DC9) => {
        29
    };
    (C20DC11) => {
        30
    };
    (C20DC11DC14) => {
        31
    };
    (C20DC5DC8DC11) => {
        32
    };
    (C20DC8DC11DC14) => {
        33
    };
    (C20DC11DC14DC17) => {
        34
    };
    (C20DC5DC8DC11DC14) => {
        35
    };
    (C20DC8DC11DC14DC17) => {
        36
    };
    (C20DC5DC8DC11DC14DC17) => {
        37
    };
    (C21) => {
        38
    };
    (C22) => {
        39
    };
    (C22DC13) => {
        40
    };
    (C22DC13DC16) => {
        41
    };
    (C22DC5DC13DC16) => {
        42
    };
    (C22DC7DC10DC13DC16) => {
        43
    };
    (C22DC7DC10DC13DC16DC19) => {
        44
    };
    (C22DC4DC7DC10DC13DC16DC19) => {
        45
    };
    (C23) => {
        46
    };
    (C24) => {
        47
    };
    (C24DC15) => {
        48
    };
    (C24DC15DC18) => {
        49
    };
    (C24DC12DC15DC18) => {
        50
    };
    (C24DC9DC12DC15DC18) => {
        51
    };
    (C24DC6DC9DC12DC15DC18) => {
        52
    };
    (C24DC6DC9DC12DC15DC18DC21) => {
        53
    };
    (C25) => {
        54
    };
    (C26) => {
        55
    };
    (C26DC17) => {
        56
    };
    (C27) => {
        57
    };
    (C28) => {
        58
    };
    (C29) => {
        59
    };
    (C30) => {
        60
    };
    (C30DC21) => {
        61
    };
    (C31) => {
        62
    };
    (C32) => {
        63
    };
    (C33) => {
        64
    };
    (C34) => {
        65
    };
    (C35) => {
        66
    };
    (C36) => {
        67
    };
}

// const BU: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Bu", &C4));
// const V: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("V", &C5));
// const CO: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Co", &C6));
// const EN: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("En", &C7));
// const CY: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Cy", &C8));
// const CA: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Ca", &C10));
// const LA: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("La", &C12));
// const M: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("M", &C14));
// const P: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("P", &C16));
// const PO: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Po", &C16DC9));
// const PE: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Pe", &C16DT9));
// const S: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("S", &C18));
// const O: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("O", &C18DC9));
// const EL: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("El", &C18DT9));
// const L: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("L", &C18DC9DC12));
// const LN: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Ln", &C18DC6DC9DC12));
// const GLN: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Gln", &C18DC6DC9DC12));
// const EO: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Eo", &C18DC9DT11DT13));
// const JA: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Ja", &C18DT9DT11DT13));
// const CT: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Ct", &C18DT9DT11DC13));
// const ST: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("St", &C18DC6DC9DC12DC15));
// const A: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("A", &C20));
// const G: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("G", &C20DC9));
// const GO: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Go", &C20DC11));
// const AO: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Ao", &C20DC5DC8DC11DC14));
// const EP: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Ep", &C20DC5DC8DC11DC14DC17));
// const B: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("B", &C22));
// const E: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("E", &C22DC13));
// const DP: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Dp", &C22DC7DC10DC13DC16DC19));
// const DH: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Dh", &C22DC4DC7DC10DC13DC16DC19));
// const LG: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("Lg", &C24));
// const N: LazyLock<(&str, &AnyValue<'static>)> = LazyLock::new(|| ("N", &C24DC15));

const BU: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C4);
const V: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C5);
const CO: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C6);
const EN: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C7);
const CY: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C8);
const CA: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C10);
const LA: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C12);
const M: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C14);
const P: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C16);
const PO: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C16DC9);
const PE: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C16DT9);
const S: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C18);
const O: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C18DC9);
const EL: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C18DT9);
const L: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C18DC9DC12);
const LN: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C18DC6DC9DC12);
const GLN: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C18DC6DC9DC12);
const EO: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C18DC9DT11DT13);
const JA: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C18DT9DT11DT13);
const CT: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C18DT9DT11DC13);
const ST: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C18DC6DC9DC12DC15);
const A: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C20);
const G: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C20DC9);
const GO: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C20DC11);
const AO: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C20DC5DC8DC11DC14);
const EP: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C20DC5DC8DC11DC14DC17);
const B: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C22);
const E: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C22DC13);
const DP: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C22DC7DC10DC13DC16DC19);
const DH: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C22DC4DC7DC10DC13DC16DC19);
const LG: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C24);
const N: LazyLock<&AnyValue<'static>> = LazyLock::new(|| &C24DC15);

/// [byrdwell.com](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
fn fatty_acids() -> PolarsResult<DataFrame> {
    df! {
        FATTY_ACID => Series::from_any_values_and_dtype(FATTY_ACID.into(), &[
            fatty_acid!(C4 { })?, // 0
            fatty_acid!(C5 { })?, // 1
            fatty_acid!(C6 { })?, // 2
            fatty_acid!(C7 { })?, // 3
            fatty_acid!(C8 { })?, // 4
            fatty_acid!(C9 { })?, // 5
            fatty_acid!(C10 { })?, // 6
            fatty_acid!(C11 { })?, // 7
            fatty_acid!(C12 { })?, // 8
            fatty_acid!(C13 { })?, // 9
            fatty_acid!(C14 { })?, // 10
            fatty_acid!(C15 { })?, // 11
            fatty_acid!(C16 { })?, // 12
            fatty_acid!(C16 { 9 => C })?, // 13
            fatty_acid!(C16 { 9 => T })?, // 14
            fatty_acid!(C17 { })?, // 15
            fatty_acid!(C18 { })?, // 16
            fatty_acid!(C18 { 9 => C })?, // 17
            fatty_acid!(C18 { 9 => T })?, // 18
            fatty_acid!(C18 { 9 => C, 12 => C })?, // 19
            fatty_acid!(C18 { 6 => C, 9 => C, 12 => C })?, // 20
            fatty_acid!(C18 { 8 => C, 10 => T, 12 => C })?, // 21
            fatty_acid!(C18 { 9 => C, 12 => C, 15 => C })?, // 22
            fatty_acid!(C18 { 9 => C, 11 => T, 13 => T })?, // 23
            fatty_acid!(C18 { 9 => T, 11 => T, 13 => C })?, // 24
            fatty_acid!(C18 { 9 => T, 11 => T, 13 => T })?, // 25
            fatty_acid!(C18 { 6 => C, 9 => C, 12 => C, 15 => C })?, // 26
            fatty_acid!(C19 { })?, // 27
            fatty_acid!(C20 { })?, // 28
            fatty_acid!(C20 { 9 => C })?, // 29
            fatty_acid!(C20 { 11 => C })?, // 30
            fatty_acid!(C20 { 11 => C, 14 => C })?, // 31
            fatty_acid!(C20 { 5 => C, 8 => C, 11 => C })?, // 32
            fatty_acid!(C20 { 8 => C, 11 => C, 14 => C })?, // 33
            fatty_acid!(C20 { 11 => C, 14 => C, 17 => C })?, // 34
            fatty_acid!(C20 { 5 => C, 8 => C, 11 => C, 14 => C })?, // 35
            fatty_acid!(C20 { 8 => C, 11 => C, 14 => C, 17 => C })?, // 36
            fatty_acid!(C20 { 5 => C, 8 => C, 11 => C, 14 => C, 17 => C })?, // 37
            fatty_acid!(C21 { })?, // 38
            fatty_acid!(C22 { })?, // 39
            fatty_acid!(C22 { 13 => C })?, // 40
            fatty_acid!(C22 { 13 => C, 16 => C })?, // 41
            fatty_acid!(C22 { 5 => C, 13 => C, 16 => C })?, // 42
            fatty_acid!(C22 { 7 => C, 10 => C, 13 => C, 16 => C })?, // 43
            fatty_acid!(C22 { 7 => C, 10 => C, 13 => C, 16 => C, 19 => C })?, // 44
            fatty_acid!(C22 { 4 => C, 7 => C, 10 => C, 13 => C, 16 => C, 19 => C })?, // 45
            fatty_acid!(C23 { })?, // 46
            fatty_acid!(C24 { })?, // 47
            fatty_acid!(C24 { 15 => C })?, // 48
            fatty_acid!(C24 { 15 => C, 18 => C })?, // 49
            fatty_acid!(C24 { 12 => C, 15 => C, 18 => C })?, // 50
            fatty_acid!(C24 { 9 => C, 12 => C, 15 => C, 18 => C })?, // 51
            fatty_acid!(C24 { 6 => C, 9 => C, 12 => C, 15 => C, 18 => C })?, // 52
            fatty_acid!(C24 { 6 => C, 9 => C, 12 => C, 15 => C, 18 => C, 21 => C })?, // 53
            fatty_acid!(C25 { })?, // 54
            fatty_acid!(C26 { })?, // 55
            fatty_acid!(C26 { 17 => C })?, // 56
            fatty_acid!(C27 { })?, // 57
            fatty_acid!(C28 { })?, // 58
            fatty_acid!(C29 { })?, // 59
            fatty_acid!(C30 { })?, // 60
            fatty_acid!(C30 { 21 => C })?, // 61
            fatty_acid!(C31 { })?, // 62
            fatty_acid!(C32 { })?, // 63
            fatty_acid!(C33 { })?, // 64
            fatty_acid!(C34 { })?, // 65
            fatty_acid!(C35 { })?, // 66
            fatty_acid!(C36 { })?, // 67
        ], &data_type!(FATTY_ACID), true)?,
    }
}

fn fatty_acids_with_row_index() -> PolarsResult<DataFrame> {
    fatty_acids()?.with_row_index(PlSmallStr::from_static("Index"), None)
}

fn fatty_acids_with_float() -> PolarsResult<DataFrame> {
    Ok(fatty_acids()?
        .with_column(Series::new(
            "Float".into(),
            &[
                0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0,
                15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0,
                29.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0,
                43.0, 44.0, 45.0, 46.0, 47.0, 48.0, 49.0, 50.0, 51.0, 52.0, 53.0, 54.0, 55.0, 56.0,
                57.0, 58.0, 59.0, 60.0, 61.0, 62.0, 63.0, 64.0, 65.0, 66.0, 67.0,
            ],
        ))?
        .clone())
}

fn fatty_acid(any_value: AnyValue) -> PolarsResult<DataFrame> {
    df! {
        FATTY_ACID => Series::from_any_values_and_dtype(FATTY_ACID.into(), &[
            any_value,
        ], &data_type!(FATTY_ACID), true)?,
    }
}

fn triacylglycerols() -> PolarsResult<DataFrame> {
    df! {
        TRIACYLGLYCEROL => Series::from_any_values_and_dtype(TRIACYLGLYCEROL.into(), &[
            triacylglycerol(&LA, &M, &P), // 0
            triacylglycerol(&LA, &M, &PO), // 1
            triacylglycerol(&LA, &M, &S), // 2
            triacylglycerol(&LA, &M, &O), // 3
            triacylglycerol(&LA, &M, &L), // 4
            triacylglycerol(&LA, &M, &LN), // 5
            triacylglycerol(&LA, &M, &A), // 6
            triacylglycerol(&LA, &P, &PO), // 7
            triacylglycerol(&LA, &P, &S), // 8
            triacylglycerol(&LA, &P, &O), // 9
            triacylglycerol(&LA, &P, &L), // 10
            triacylglycerol(&LA, &P, &LN), // 11
            triacylglycerol(&LA, &P, &A), // 12
            triacylglycerol(&LA, &PO, &S), // 13
            triacylglycerol(&LA, &PO, &O), // 14
            triacylglycerol(&LA, &PO, &L), // 15
            triacylglycerol(&LA, &PO, &LN), // 16
            triacylglycerol(&LA, &PO, &A), // 17
            triacylglycerol(&LA, &S, &O), // 18
            triacylglycerol(&LA, &S, &L), // 19
            triacylglycerol(&LA, &S, &LN), // 20
            triacylglycerol(&LA, &S, &A), // 21
            triacylglycerol(&LA, &O, &L), // 22
            triacylglycerol(&LA, &O, &LN), // 23
            triacylglycerol(&LA, &O, &A), // 24
            triacylglycerol(&LA, &L, &LN), // 25
            triacylglycerol(&LA, &L, &A), // 26
            triacylglycerol(&M, &P, &PO), // 27
            triacylglycerol(&M, &P, &S), // 28
            triacylglycerol(&M, &P, &O), // 29
            triacylglycerol(&M, &P, &L), // 30
            triacylglycerol(&M, &P, &LN), // 31
            triacylglycerol(&M, &P, &A), // 32
            triacylglycerol(&M, &PO, &S), // 33
            triacylglycerol(&M, &PO, &O), // 34
            triacylglycerol(&M, &PO, &L), // 35
            triacylglycerol(&M, &PO, &LN), // 36
            triacylglycerol(&M, &PO, &A), // 37
            triacylglycerol(&M, &S, &O), // 38
            triacylglycerol(&M, &S, &L), // 39
            triacylglycerol(&M, &S, &LN), // 40
            triacylglycerol(&M, &S, &A), // 41
            triacylglycerol(&M, &O, &L), // 42
            triacylglycerol(&M, &O, &LN), // 43
            triacylglycerol(&M, &O, &A), // 44
            triacylglycerol(&M, &L, &LN), // 45
            triacylglycerol(&M, &L, &A), // 46
            triacylglycerol(&M, &LN, &A), // 47
            triacylglycerol(&P, &PO, &S), // 48
            triacylglycerol(&P, &PO, &O), // 49
            triacylglycerol(&P, &PO, &L), // 50
            triacylglycerol(&P, &PO, &LN), // 51
            triacylglycerol(&P, &PO, &A), // 52
            triacylglycerol(&P, &S, &O), // 53
            triacylglycerol(&P, &S, &L), // 54
            triacylglycerol(&P, &S, &LN), // 55
            triacylglycerol(&P, &S, &A), // 56
            triacylglycerol(&P, &O, &L), // 57
            triacylglycerol(&P, &O, &LN), // 58
            triacylglycerol(&P, &O, &A), // 59
            triacylglycerol(&P, &L, &LN), // 60
            triacylglycerol(&P, &L, &A), // 61
            triacylglycerol(&P, &LN, &A), // 62
            triacylglycerol(&PO, &S, &O), // 63
            triacylglycerol(&PO, &S, &L), // 64
            triacylglycerol(&PO, &S, &LN), // 65
            triacylglycerol(&PO, &S, &A), // 66
            triacylglycerol(&PO, &O, &L), // 67
            triacylglycerol(&PO, &O, &LN), // 68
            triacylglycerol(&PO, &L, &A), // 69
            triacylglycerol(&PO, &L, &LN), // 70
            triacylglycerol(&PO, &L, &A), // 71
            triacylglycerol(&PO, &LN, &A), // 72
            triacylglycerol(&S, &O, &L), // 73
            triacylglycerol(&S, &O, &LN), // 74
            triacylglycerol(&S, &O, &A), // 75
            triacylglycerol(&S, &L, &LN), // 76
            triacylglycerol(&S, &L, &A), // 77
            triacylglycerol(&S, &LN, &A), // 78
            triacylglycerol(&O, &L, &LN), // 79
            triacylglycerol(&O, &L, &A), // 80
            triacylglycerol(&O, &LN, &A), // 81
            triacylglycerol(&L, &LN, &A), // 82
        ], &data_type!(TRIACYLGLYCEROL), true)?,
    }
}

fn triacylglycerols_with_float() -> PolarsResult<DataFrame> {
    Ok(triacylglycerols()?
        .with_column(Series::new(
            "Float".into(),
            &[
                0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0,
                15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0,
                29.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0,
                43.0, 44.0, 45.0, 46.0, 47.0, 48.0, 49.0, 50.0, 51.0, 52.0, 53.0, 54.0, 55.0, 56.0,
                57.0, 58.0, 59.0, 60.0, 61.0, 62.0, 63.0, 64.0, 65.0, 66.0, 67.0, 68.0, 69.0, 70.0,
                71.0, 72.0, 73.0, 74.0, 75.0, 76.0, 77.0, 78.0, 79.0, 80.0, 81.0, 82.0,
            ],
        ))?
        .clone())
}

fn triacylglycerol(
    sn1: &AnyValue<'static>,
    sn2: &AnyValue<'static>,
    sn3: &AnyValue<'static>,
) -> AnyValue<'static> {
    AnyValue::StructOwned(Box::new((
        vec![sn1.clone(), sn2.clone(), sn3.clone()],
        vec![
            Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS1),
                data_type!(FATTY_ACID),
            ),
            Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS2),
                data_type!(FATTY_ACID),
            ),
            Field::new(
                PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS3),
                data_type!(FATTY_ACID),
            ),
        ],
    )))
}

// fn tags_with_label() -> PolarsResult<DataFrame> {
//     df! {
//         TRIACYLGLYCEROL => Series::from_any_values_and_dtype(TRIACYLGLYCEROL.into(), &[
//             tag_with_label(&LA, &M, &P), // 0
//             tag_with_label(&LA, &M, &PO), // 1
//             tag_with_label(&LA, &M, &S), // 2
//             tag_with_label(&LA, &M, &O), // 3
//             tag_with_label(&LA, &M, &L), // 4
//             tag_with_label(&LA, &M, &LN), // 5
//             tag_with_label(&LA, &M, &A), // 6
//             tag_with_label(&LA, &P, &PO), // 7
//             tag_with_label(&LA, &P, &S), // 8
//             tag_with_label(&LA, &P, &O), // 9
//             tag_with_label(&LA, &P, &L), // 10
//             tag_with_label(&LA, &P, &LN), // 11
//             tag_with_label(&LA, &P, &A), // 12
//             tag_with_label(&LA, &PO, &S), // 13
//             tag_with_label(&LA, &PO, &O), // 14
//             tag_with_label(&LA, &PO, &L), // 15
//             tag_with_label(&LA, &PO, &LN), // 16
//             tag_with_label(&LA, &PO, &A), // 17
//             tag_with_label(&LA, &S, &O), // 18
//             tag_with_label(&LA, &S, &L), // 19
//             tag_with_label(&LA, &S, &LN), // 20
//             tag_with_label(&LA, &S, &A), // 21
//             tag_with_label(&LA, &O, &L), // 22
//             tag_with_label(&LA, &O, &LN), // 23
//             tag_with_label(&LA, &O, &A), // 24
//             tag_with_label(&LA, &L, &LN), // 25
//             tag_with_label(&LA, &L, &A), // 26
//             tag_with_label(&M, &P, &PO), // 27
//             tag_with_label(&M, &P, &S), // 28
//             tag_with_label(&M, &P, &O), // 29
//             tag_with_label(&M, &P, &L), // 30
//             tag_with_label(&M, &P, &LN), // 31
//             tag_with_label(&M, &P, &A), // 32
//             tag_with_label(&M, &PO, &S), // 33
//             tag_with_label(&M, &PO, &O), // 34
//             tag_with_label(&M, &PO, &L), // 35
//             tag_with_label(&M, &PO, &LN), // 36
//             tag_with_label(&M, &PO, &A), // 37
//             tag_with_label(&M, &S, &O), // 38
//             tag_with_label(&M, &S, &L), // 39
//             tag_with_label(&M, &S, &LN), // 40
//             tag_with_label(&M, &S, &A), // 41
//             tag_with_label(&M, &O, &L), // 42
//             tag_with_label(&M, &O, &LN), // 43
//             tag_with_label(&M, &O, &A), // 44
//             tag_with_label(&M, &L, &LN), // 45
//             tag_with_label(&M, &L, &A), // 46
//             tag_with_label(&M, &LN, &A), // 47
//             tag_with_label(&P, &PO, &S), // 48
//             tag_with_label(&P, &PO, &O), // 49
//             tag_with_label(&P, &PO, &L), // 50
//             tag_with_label(&P, &PO, &LN), // 51
//             tag_with_label(&P, &PO, &A), // 52
//             tag_with_label(&P, &S, &O), // 53
//             tag_with_label(&P, &S, &L), // 54
//             tag_with_label(&P, &S, &LN), // 55
//             tag_with_label(&P, &S, &A), // 56
//             tag_with_label(&P, &O, &L), // 57
//             tag_with_label(&P, &O, &LN), // 58
//             tag_with_label(&P, &O, &A), // 59
//             tag_with_label(&P, &L, &LN), // 60
//             tag_with_label(&P, &L, &A), // 61
//             tag_with_label(&P, &LN, &A), // 62
//             tag_with_label(&PO, &S, &O), // 63
//             tag_with_label(&PO, &S, &L), // 64
//             tag_with_label(&PO, &S, &LN), // 65
//             tag_with_label(&PO, &S, &A), // 66
//             tag_with_label(&PO, &O, &L), // 67
//             tag_with_label(&PO, &O, &LN), // 68
//             tag_with_label(&PO, &L, &A), // 69
//             tag_with_label(&PO, &L, &LN), // 70
//             tag_with_label(&PO, &L, &A), // 71
//             tag_with_label(&PO, &LN, &A), // 72
//             tag_with_label(&S, &O, &L), // 73
//             tag_with_label(&S, &O, &LN), // 74
//             tag_with_label(&S, &O, &A), // 75
//             tag_with_label(&S, &L, &LN), // 76
//             tag_with_label(&S, &L, &A), // 77
//             tag_with_label(&S, &LN, &A), // 78
//             tag_with_label(&O, &L, &LN), // 79
//             tag_with_label(&O, &L, &A), // 80
//             tag_with_label(&O, &LN, &A), // 81
//             tag_with_label(&L, &LN, &A), // 82
//         ], &data_type!(TRIACYLGLYCEROL), true)?,
//     }
// }

// fn tag_with_label(
//     sn1: &(&'static str, &AnyValue<'static>),
//     sn2: &(&'static str, &AnyValue<'static>),
//     sn3: &(&'static str, &AnyValue<'static>),
// ) -> AnyValue<'static> {
//     AnyValue::StructOwned(Box::new((
//         vec![
//             AnyValue::StructOwned(Box::new((
//                 vec![AnyValue::String(sn1.0), sn1.1.clone()],
//                 vec![field!(LABEL), field!(FATTY_ACID)],
//             ))),
//             AnyValue::StructOwned(Box::new((
//                 vec![AnyValue::String(sn2.0), sn2.1.clone()],
//                 vec![field!(LABEL), field!(FATTY_ACID)],
//             ))),
//             AnyValue::StructOwned(Box::new((
//                 vec![AnyValue::String(sn3.0), sn3.1.clone()],
//                 vec![field!(LABEL), field!(FATTY_ACID)],
//             ))),
//         ],
//         vec![
//             Field::new(
//                 PlSmallStr::from_static(STEREOSPECIFIC_NUMBER1),
//                 data_type!(STEREOSPECIFIC_NUMBER),
//             ),
//             Field::new(
//                 PlSmallStr::from_static(STEREOSPECIFIC_NUMBER2),
//                 data_type!(STEREOSPECIFIC_NUMBER),
//             ),
//             Field::new(
//                 PlSmallStr::from_static(STEREOSPECIFIC_NUMBER3),
//                 data_type!(STEREOSPECIFIC_NUMBER),
//             ),
//         ],
//     )))
// }

/// Round a value to the given number of decimal places.
fn round_to_decimals(value: f64, decimals: usize) -> f64 {
    format!("{value:.decimals$}").parse().unwrap()
}

mod chunked_array;
mod expr;
