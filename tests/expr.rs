use lipid::fatty_acid::polars::prelude::*;
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
