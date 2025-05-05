use super::fatty_acid;
use anyhow::Result;
use lipid::{polars::FATTY_ACID, prelude::*};
use polars::prelude::*;
use std::sync::LazyLock;

const DECIMALS: usize = 4;

/// [byrdwell.com](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static FATTY_ACIDS: LazyLock<DataFrame> = LazyLock::new(|| {
    fatty_acids()
        .unwrap()
        .with_row_index(PlSmallStr::from_static("Index"), None)
        .unwrap()
});

fn fatty_acids() -> PolarsResult<DataFrame> {
    df! {
        "FattyAcid" => [
            FattyAcidChunked::try_from(&C4)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 0
            FattyAcidChunked::try_from(&C5)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 1
            FattyAcidChunked::try_from(&C6)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 2
            FattyAcidChunked::try_from(&C7)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 3
            FattyAcidChunked::try_from(&C8)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 4
            FattyAcidChunked::try_from(&C9)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 5
            FattyAcidChunked::try_from(&C10)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 6
            FattyAcidChunked::try_from(&C11)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 7
            FattyAcidChunked::try_from(&C12)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 8
            FattyAcidChunked::try_from(&C13)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 9
            FattyAcidChunked::try_from(&C14)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 10
            FattyAcidChunked::try_from(&C15)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 11
            FattyAcidChunked::try_from(&C16)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 12
            FattyAcidChunked::try_from(&C16DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 13
            FattyAcidChunked::try_from(&C16DT9)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 14
            FattyAcidChunked::try_from(&C17)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 15
            FattyAcidChunked::try_from(&C18)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 16
            FattyAcidChunked::try_from(&C18DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 17
            FattyAcidChunked::try_from(&C18DT9)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 18
            FattyAcidChunked::try_from(&C18DC9DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 19
            FattyAcidChunked::try_from(&C18DC6DC9DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 20
            FattyAcidChunked::try_from(&C18DC8DT10DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 21
            FattyAcidChunked::try_from(&C18DC9DC12DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 22
            FattyAcidChunked::try_from(&C18DC9DT11DT13)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 23
            FattyAcidChunked::try_from(&C18DT9DT11DC13)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 24
            FattyAcidChunked::try_from(&C18DT9DT11DT13)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 25
            FattyAcidChunked::try_from(&C18DC6DC9DC12DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 26
            FattyAcidChunked::try_from(&C19)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 27
            FattyAcidChunked::try_from(&C20)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 28
            FattyAcidChunked::try_from(&C20DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 29
            FattyAcidChunked::try_from(&C20DC11)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 30
            FattyAcidChunked::try_from(&C20DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 31
            FattyAcidChunked::try_from(&C20DC5DC8DC11)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 32
            FattyAcidChunked::try_from(&C20DC8DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 33
            FattyAcidChunked::try_from(&C20DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 34
            FattyAcidChunked::try_from(&C20DC5DC8DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 35
            FattyAcidChunked::try_from(&C20DC8DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 36
            FattyAcidChunked::try_from(&C20DC5DC8DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 37
            FattyAcidChunked::try_from(&C21)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 38
            FattyAcidChunked::try_from(&C22)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 39
            FattyAcidChunked::try_from(&C22DC13)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 40
            FattyAcidChunked::try_from(&C22DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 41
            FattyAcidChunked::try_from(&C22DC5DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 42
            FattyAcidChunked::try_from(&C22DC7DC10DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 43
            FattyAcidChunked::try_from(&C22DC7DC10DC13DC16DC19)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 44
            FattyAcidChunked::try_from(&C22DC4DC7DC10DC13DC16DC19)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 45
            FattyAcidChunked::try_from(&C23)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 46
            FattyAcidChunked::try_from(&C24)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 47
            FattyAcidChunked::try_from(&C24DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 48
            FattyAcidChunked::try_from(&C24DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 49
            FattyAcidChunked::try_from(&C24DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 50
            FattyAcidChunked::try_from(&C24DC9DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 51
            FattyAcidChunked::try_from(&C24DC6DC9DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 52
            FattyAcidChunked::try_from(&C24DC6DC9DC12DC15DC18DC21)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 53
            FattyAcidChunked::try_from(&C25)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 54
            FattyAcidChunked::try_from(&C26)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 55
            FattyAcidChunked::try_from(&C26DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 56
            FattyAcidChunked::try_from(&C27)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 57
            FattyAcidChunked::try_from(&C28)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 58
            FattyAcidChunked::try_from(&C29)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 59
            FattyAcidChunked::try_from(&C30)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 60
            FattyAcidChunked::try_from(&C30DC21)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 61
            FattyAcidChunked::try_from(&C31)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 62
            FattyAcidChunked::try_from(&C32)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 63
            FattyAcidChunked::try_from(&C33)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 64
            FattyAcidChunked::try_from(&C34)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 65
            FattyAcidChunked::try_from(&C35)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 66
            FattyAcidChunked::try_from(&C36)?.into_struct(PlSmallStr::EMPTY)?.into_series(), // 67
        ],
    }
}

/// Round a value to the given number of decimal places.
fn round_to_decimals(value: f64, decimals: usize) -> f64 {
    format!("{value:.decimals$}").parse().unwrap_or(value)
}

#[test]
fn test1() -> Result<()> {
    // unsafe { std::env::set_var("POLARS_FMT_MAX_ROWS", "256") };
    unsafe { std::env::set_var("POLARS_FMT_TABLE_CELL_LIST_LEN", "1024") };
    unsafe { std::env::set_var("POLARS_FMT_STR_LEN", "1024") };

    let fatty_acids = &*FATTY_ACIDS;
    println!("fatty_acids: {fatty_acids}");
    let series = FATTY_ACIDS[FATTY_ACID].as_materialized_series();
    println!("series: {series}");
    let list = FATTY_ACIDS.try_fatty_acid_list()?;
    // println!("inner_dtype: {}", list.inner_dtype());
    for fatty_acid in list {
        println!("fatty_acid: {fatty_acid} {}", fatty_acid.mass(None));
    }
    // let c = IndexedIdentifierListChunked::new(list.clone());
    // println!("c: {c:?}");
    // let chunked = list.clone().fatty_acid()?;
    // println!("c2: {chunked:?}");
    // let next = FATTY_ACIDS[FATTY_ACID]
    //     .fatty_acid()
    //     .into_iter()
    //     .next()
    //     .unwrap();
    // println!("next: {next}");

    // println!(
    //     "fatty_acid: {}",
    //     FATTY_ACIDS.fatty_acid().iter().next().unwrap()
    // );

    Ok(())
}

// mod bound;
// mod fatty_acid;
#[cfg(feature = "mass")]
mod mass;
