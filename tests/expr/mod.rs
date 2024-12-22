use lipid::fatty_acid::polars::{expr::chain_length::Options, prelude::*};
use polars::prelude::*;
use std::iter::empty;

#[test]
fn find() -> PolarsResult<()> {
    println!("C18U3Z9E11E13: {:#?}", &*C18U3Z9E11E13);
    println!("C18U2Z9Z12: {:#?}", &*C18U2Z9Z12);

    let data_frame = df! {
        "FattyAcid" => df! {
            "Carbons" => &[
                16u8,
                18u8,
                18u8,
                18u8,
                18u8,
            ],
            "Unsaturated" => &[
                df! {
                    "Index"        => Series::from_iter(empty::<u8>()),
                    "Isomerism"    => Series::from_iter(empty::<i8>()),
                    "Unsaturation" => Series::from_iter(empty::<u8>()),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
                df! {
                    "Index"        => Series::from_iter(empty::<u8>()),
                    "Isomerism"    => Series::from_iter(empty::<i8>()),
                    "Unsaturation" => Series::from_iter(empty::<u8>()),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
                df! {
                    "Index"        => Series::from_iter([9u8]),
                    "Isomerism"    => Series::from_iter([1i8]),
                    "Unsaturation" => Series::from_iter([1u8]),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
                df! {
                    "Index"        => Series::from_iter([9u8, 12]),
                    "Isomerism"    => Series::from_iter([1i8, 1]),
                    "Unsaturation" => Series::from_iter([1u8, 1]),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
                df! {
                    "Index"        => Series::from_iter([9u8, 12, 15]),
                    "Isomerism"    => Series::from_iter([1i8, 1, 1]),
                    "Unsaturation" => Series::from_iter([1u8, 1, 1]),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
            ],
        }?.into_struct(PlSmallStr::EMPTY),
        "Value" => [
            Some(1f64),
            Some(2f64),
            Some(3f64),
            Some(4f64),
            Some(5f64),
        ],

    }?;
    println!("data_frame: {}", data_frame);
    println!("data_frame: {}", data_frame.unnest(["FattyAcid"])?);
    let mut lazy_frame = data_frame.lazy();
    lazy_frame = lazy_frame.with_columns([
        col("Value")
            .filter(col("FattyAcid").fatty_acid().equal(C18U0.clone()))
            .sum()
            .alias("C18U0"),
        col("Value")
            .filter(col("FattyAcid").fatty_acid().eicosapentaenoic())
            .sum()
            .alias("Eicosapentaenoic"),
        col("Value")
            .filter(col("FattyAcid").fatty_acid().equal(C18U1Z9.clone()))
            .sum()
            .alias("C18U3Z9Z12Z15"),
    ]);
    println!("lazy_frame: {}", lazy_frame.clone().collect().unwrap());
    // assert_eq!(EXPECTED_DEBUG_F, format!("{:?}", f().unwrap_err()));
    // assert_eq!(EXPECTED_DEBUG_G, format!("{:?}", g().unwrap_err()));
    // assert_eq!(EXPECTED_DEBUG_H, format!("{:?}", h().unwrap_err()));
    Ok(())
}

#[test]
fn filter_unsaturated() -> PolarsResult<()> {
    let data_frame = df! {
        "FattyAcid" => df! {
            "Carbons" => &[
                16u8,
                18u8,
                18u8,
                18u8,
                18u8,
                20u8,
            ],
            "Unsaturated" => &[
                df! {
                    "Index"        => Series::from_iter(empty::<u8>()),
                    "Isomerism"    => Series::from_iter(empty::<i8>()),
                    "Unsaturation" => Series::from_iter(empty::<u8>()),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
                df! {
                    "Index"        => Series::from_iter(empty::<u8>()),
                    "Isomerism"    => Series::from_iter(empty::<i8>()),
                    "Unsaturation" => Series::from_iter(empty::<u8>()),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
                df! {
                    "Index"        => Series::from_iter([9u8]),
                    "Isomerism"    => Series::from_iter([1i8]),
                    "Unsaturation" => Series::from_iter([1u8]),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
                df! {
                    "Index"        => Series::from_iter([9u8, 12]),
                    "Isomerism"    => Series::from_iter([1i8, 1]),
                    "Unsaturation" => Series::from_iter([1u8, 1]),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
                df! {
                    "Index"        => Series::from_iter([9u8, 12, 15]),
                    "Isomerism"    => Series::from_iter([1i8, 1, 1]),
                    "Unsaturation" => Series::from_iter([1u8, 1, 1]),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
                df! {
                    "Index"        => Series::from_iter(empty::<u8>()),
                    "Isomerism"    => Series::from_iter(empty::<i8>()),
                    "Unsaturation" => Series::from_iter(empty::<u8>()),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
            ],
        }?.into_struct(PlSmallStr::EMPTY),
        "Value" => [
            Some(1f64),
            Some(2f64),
            Some(3f64),
            Some(4f64),
            Some(5f64),
            Some(6f64),
        ],
        "RetentionTime" => &[
            10f64,
            11f64,
            11.25f64,
            11.5f64,
            11.75f64,
            12f64,
        ],
    }?;
    println!("data_frame: {}", data_frame);
    println!("data_frame: {}", data_frame.unnest(["FattyAcid"])?);
    let mut lazy_frame = data_frame.lazy();
    lazy_frame = lazy_frame.with_columns([
        col("FattyAcid")
            .fatty_acid()
            .fcl(col("RetentionTime"), Default::default())
            .alias("FCL0"),
        col("FattyAcid")
            .fatty_acid()
            .fcl(col("RetentionTime"), Options::new().logarithmic(true))
            .alias("FCL1"),
        col("FattyAcid")
            .fatty_acid()
            .ecl(col("RetentionTime"), Options::default())
            .alias("ECL"),
        col("FattyAcid")
            .fatty_acid()
            .ecl(col("RetentionTime"), Options::new().logarithmic(true))
            .alias("ECL1"),
    ]);
    // .map_saturated_or(lit(0), |saturated| {
    //     (col("TimeMean")
    //         - col("TimeMean")
    //             .filter(col("FattyAcid").fatty_acid())
    //             .forward_fill(None))
    //         / (col("TimeMean")
    //             .filter(col("FattyAcid").fatty_acid())
    //             .backward_fill(None)
    //             - col("TimeMean")
    //                 .filter(col("FattyAcid").fatty_acid())
    //                 .forward_fill(None))
    // })
    // .alias("Saturated")]);
    // saturated(expr.clone()).backward_fill(None) - saturated(expr).forward_fill(None)
    println!("lazy_frame: {}", lazy_frame.clone().collect()?);
    Ok(())
}

#[cfg(test)]
mod ecn;
