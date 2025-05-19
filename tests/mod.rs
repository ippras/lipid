//! [byrdwell.com](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)

use lipid::prelude::*;
use polars::prelude::*;
use std::num::NonZero;

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

fn fatty_acid<T>(fatty_acid: T) -> PolarsResult<DataFrame>
where
    T: TryInto<FattyAcidChunked, Error = PolarsError>,
{
    df! {
        "FattyAcid" => [
            fatty_acid.try_into()?.into_struct(PlSmallStr::EMPTY)?.into_series(),
        ],
    }
}

fn fatty_acids() -> PolarsResult<DataFrame> {
    df! {
        "FattyAcid" => [
            Some(FattyAcidChunked::try_from(C4)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 0
            Some(FattyAcidChunked::try_from(C5)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 1
            Some(FattyAcidChunked::try_from(C6)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 2
            Some(FattyAcidChunked::try_from(C7)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 3
            Some(FattyAcidChunked::try_from(C8)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 4
            Some(FattyAcidChunked::try_from(C9)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 5
            Some(FattyAcidChunked::try_from(C10)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 6
            Some(FattyAcidChunked::try_from(C11)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 7
            Some(FattyAcidChunked::try_from(C12)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 8
            Some(FattyAcidChunked::try_from(C13)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 9
            Some(FattyAcidChunked::try_from(C14)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 10
            Some(FattyAcidChunked::try_from(C15)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 11
            Some(FattyAcidChunked::try_from(C16)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 12
            Some(FattyAcidChunked::try_from(C16DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 13
            Some(FattyAcidChunked::try_from(C16DT9)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 14
            Some(FattyAcidChunked::try_from(C17)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 15
            Some(FattyAcidChunked::try_from(C18)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 16
            Some(FattyAcidChunked::try_from(C18DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 17
            Some(FattyAcidChunked::try_from(C18DT9)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 18
            Some(FattyAcidChunked::try_from(C18DC9DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 19
            Some(FattyAcidChunked::try_from(C18DC6DC9DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 20
            Some(FattyAcidChunked::try_from(C18DC8DT10DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 21
            Some(FattyAcidChunked::try_from(C18DC9DC12DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 22
            Some(FattyAcidChunked::try_from(C18DC9DT11DT13)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 23
            Some(FattyAcidChunked::try_from(C18DT9DT11DC13)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 24
            Some(FattyAcidChunked::try_from(C18DT9DT11DT13)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 25
            Some(FattyAcidChunked::try_from(C18DC6DC9DC12DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 26
            Some(FattyAcidChunked::try_from(C19)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 27
            Some(FattyAcidChunked::try_from(C20)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 28
            Some(FattyAcidChunked::try_from(C20DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 29
            Some(FattyAcidChunked::try_from(C20DC11)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 30
            Some(FattyAcidChunked::try_from(C20DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 31
            Some(FattyAcidChunked::try_from(C20DC5DC8DC11)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 32
            Some(FattyAcidChunked::try_from(C20DC8DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 33
            Some(FattyAcidChunked::try_from(C20DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 34
            Some(FattyAcidChunked::try_from(C20DC5DC8DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 35
            Some(FattyAcidChunked::try_from(C20DC8DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 36
            Some(FattyAcidChunked::try_from(C20DC5DC8DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 37
            Some(FattyAcidChunked::try_from(C21)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 38
            Some(FattyAcidChunked::try_from(C22)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 39
            Some(FattyAcidChunked::try_from(C22DC13)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 40
            Some(FattyAcidChunked::try_from(C22DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 41
            Some(FattyAcidChunked::try_from(C22DC5DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 42
            Some(FattyAcidChunked::try_from(C22DC7DC10DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 43
            Some(FattyAcidChunked::try_from(C22DC7DC10DC13DC16DC19)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 44
            Some(FattyAcidChunked::try_from(C22DC4DC7DC10DC13DC16DC19)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 45
            Some(FattyAcidChunked::try_from(C23)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 46
            Some(FattyAcidChunked::try_from(C24)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 47
            Some(FattyAcidChunked::try_from(C24DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 48
            Some(FattyAcidChunked::try_from(C24DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 49
            Some(FattyAcidChunked::try_from(C24DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 50
            Some(FattyAcidChunked::try_from(C24DC9DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 51
            Some(FattyAcidChunked::try_from(C24DC6DC9DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 52
            Some(FattyAcidChunked::try_from(C24DC6DC9DC12DC15DC18DC21)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 53
            Some(FattyAcidChunked::try_from(C25)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 54
            Some(FattyAcidChunked::try_from(C26)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 55
            Some(FattyAcidChunked::try_from(C26DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 56
            Some(FattyAcidChunked::try_from(C27)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 57
            Some(FattyAcidChunked::try_from(C28)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 58
            Some(FattyAcidChunked::try_from(C29)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 59
            Some(FattyAcidChunked::try_from(C30)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 60
            Some(FattyAcidChunked::try_from(C30DC21)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 61
            Some(FattyAcidChunked::try_from(C31)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 62
            Some(FattyAcidChunked::try_from(C32)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 63
            Some(FattyAcidChunked::try_from(C33)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 64
            Some(FattyAcidChunked::try_from(C34)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 65
            Some(FattyAcidChunked::try_from(C35)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 66
            Some(FattyAcidChunked::try_from(C36)?.into_struct(PlSmallStr::EMPTY)?.into_series()), // 67
        ],
    }?
    .with_row_index(PlSmallStr::from_static("Index"), None)
}

mod chunked_array;
mod expr;
