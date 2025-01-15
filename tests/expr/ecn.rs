use lipid::prelude::*;
use polars::prelude::*;
use std::iter::empty;

#[test]
fn find() -> PolarsResult<()> {
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
    lazy_frame = lazy_frame.with_columns([col("FattyAcid").fa().ecn().alias("ECN")]);
    println!("lazy_frame: {}", lazy_frame.clone().collect()?);
    Ok(())
}
