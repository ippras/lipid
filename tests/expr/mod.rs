use lipid::{fatty_acid::r#const::*, prelude::*};
use polars::prelude::*;
use std::sync::LazyLock;

pub static SOURCE: LazyLock<DataFrame> = LazyLock::new(|| {
    df! {
        "FattyAcid" => [
            Some(Series::from_iter(C4U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C5U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C6U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C7U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C8U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C9U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C10U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C11U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C12U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C13U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C14U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C15U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C16U1DC9).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C16U1DT9).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C17U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C18U1DT9).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C18U3DC6DC9DC12).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C18U3DC8DT10DC12).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C18U3DC9DC12DC15).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C18U3DC9DT11DT13).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C18U3DT9DT11DC13).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C18U3DT9DT11DT13).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C18U4DC6DC9DC12DC15).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C19U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C20U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C20U1DC9).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C20U1DC11).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C20U2DC11DC14).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C20U3DC5DC8DC11).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C20U3DC8DC11DC14).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C20U3DC11DC14DC17).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C20U4DC5DC8DC11DC14).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C20U4DC8DC11DC14DC17).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C20U5DC5DC8DC11DC14DC17).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C21U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C22U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C22U1DC13).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C22U2DC13DC16).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C22U3DC5DC13DC16).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C22U4DC7DC10DC13DC16).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C22U5DC7DC10DC13DC16DC19).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C22U6DC4DC7DC10DC13DC16DC19).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C23U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C24U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C24U1DC15).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C24U2DC15DC18).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C24U3DC12DC15DC18).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C24U4DC9DC12DC15DC18).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C24U5DC6DC9DC12DC15DC18).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C24U6DC6DC9DC12DC15DC18DC21).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C25U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C26U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C26U1DC17).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C27U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C28U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C29U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C30U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C30U1DC21).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C31U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C32U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C33U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C34U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C35U0).cast(&BOUND_DATA_TYPE).unwrap()),
            Some(Series::from_iter(C36U0).cast(&BOUND_DATA_TYPE).unwrap()),
            None,
        ],
    }
    .unwrap()
    .with_row_index("Index".into(), None)
    .unwrap()
});

// #[test]
// fn find() -> PolarsResult<()> {
//     println!("C18U3Z9E11E13: {:#?}", &*C18U3Z9E11E13);
//     println!("C18U2Z9Z12: {:#?}", &*C18U2Z9Z12);

//     let data_frame = df! {
//         "FattyAcid" => df! {
//             "Carbons" => &[
//                 16u8,
//                 18u8,
//                 18u8,
//                 18u8,
//                 18u8,
//             ],
//             "Unsaturated" => &[
//                 df! {
//                     "Index"        => Series::from_iter(empty::<u8>()),
//                     "Isomerism"    => Series::from_iter(empty::<i8>()),
//                     "Unsaturation" => Series::from_iter(empty::<u8>()),
//                 }?.into_struct(PlSmallStr::EMPTY).into_series(),
//                 df! {
//                     "Index"        => Series::from_iter(empty::<u8>()),
//                     "Isomerism"    => Series::from_iter(empty::<i8>()),
//                     "Unsaturation" => Series::from_iter(empty::<u8>()),
//                 }?.into_struct(PlSmallStr::EMPTY).into_series(),
//                 df! {
//                     "Index"        => Series::from_iter([9u8]),
//                     "Isomerism"    => Series::from_iter([1i8]),
//                     "Unsaturation" => Series::from_iter([1u8]),
//                 }?.into_struct(PlSmallStr::EMPTY).into_series(),
//                 df! {
//                     "Index"        => Series::from_iter([9u8, 12]),
//                     "Isomerism"    => Series::from_iter([1i8, 1]),
//                     "Unsaturation" => Series::from_iter([1u8, 1]),
//                 }?.into_struct(PlSmallStr::EMPTY).into_series(),
//                 df! {
//                     "Index"        => Series::from_iter([9u8, 12, 15]),
//                     "Isomerism"    => Series::from_iter([1i8, 1, 1]),
//                     "Unsaturation" => Series::from_iter([1u8, 1, 1]),
//                 }?.into_struct(PlSmallStr::EMPTY).into_series(),
//             ],
//         }?.into_struct(PlSmallStr::EMPTY),
//         "Value" => [
//             Some(1f64),
//             Some(2f64),
//             Some(3f64),
//             Some(4f64),
//             Some(5f64),
//         ],

//     }?;
//     println!("data_frame: {}", data_frame);
//     println!("data_frame: {}", data_frame.unnest(["FattyAcid"])?);
//     let mut lazy_frame = data_frame.lazy();
//     lazy_frame = lazy_frame.with_columns([
//         col("Value")
//             .filter(col("FattyAcid").fatty_acid().equal(C18U0.clone()))
//             .sum()
//             .alias("C18U0"),
//         col("Value")
//             .filter(col("FattyAcid").fatty_acid().eicosapentaenoic())
//             .sum()
//             .alias("Eicosapentaenoic"),
//         col("Value")
//             .filter(col("FattyAcid").fatty_acid().equal(C18U1Z9.clone()))
//             .sum()
//             .alias("C18U3Z9Z12Z15"),
//     ]);
//     println!("lazy_frame: {}", lazy_frame.clone().collect().unwrap());
//     // assert_eq!(EXPECTED_DEBUG_F, format!("{:?}", f().unwrap_err()));
//     // assert_eq!(EXPECTED_DEBUG_G, format!("{:?}", g().unwrap_err()));
//     // assert_eq!(EXPECTED_DEBUG_H, format!("{:?}", h().unwrap_err()));
//     Ok(())
// }

// #[test]
// fn filter_unsaturated() -> PolarsResult<()> {
//     let data_frame = df! {
//         "FattyAcid" => df! {
//             "Carbons" => &[
//                 16u8,
//                 18u8,
//                 18u8,
//                 18u8,
//                 18u8,
//                 20u8,
//             ],
//             "Unsaturated" => &[
//                 df! {
//                     "Index"        => Series::from_iter(empty::<u8>()),
//                     "Isomerism"    => Series::from_iter(empty::<i8>()),
//                     "Unsaturation" => Series::from_iter(empty::<u8>()),
//                 }?.into_struct(PlSmallStr::EMPTY).into_series(),
//                 df! {
//                     "Index"        => Series::from_iter(empty::<u8>()),
//                     "Isomerism"    => Series::from_iter(empty::<i8>()),
//                     "Unsaturation" => Series::from_iter(empty::<u8>()),
//                 }?.into_struct(PlSmallStr::EMPTY).into_series(),
//                 df! {
//                     "Index"        => Series::from_iter([9u8]),
//                     "Isomerism"    => Series::from_iter([1i8]),
//                     "Unsaturation" => Series::from_iter([1u8]),
//                 }?.into_struct(PlSmallStr::EMPTY).into_series(),
//                 df! {
//                     "Index"        => Series::from_iter([9u8, 12]),
//                     "Isomerism"    => Series::from_iter([1i8, 1]),
//                     "Unsaturation" => Series::from_iter([1u8, 1]),
//                 }?.into_struct(PlSmallStr::EMPTY).into_series(),
//                 df! {
//                     "Index"        => Series::from_iter([9u8, 12, 15]),
//                     "Isomerism"    => Series::from_iter([1i8, 1, 1]),
//                     "Unsaturation" => Series::from_iter([1u8, 1, 1]),
//                 }?.into_struct(PlSmallStr::EMPTY).into_series(),
//                 df! {
//                     "Index"        => Series::from_iter(empty::<u8>()),
//                     "Isomerism"    => Series::from_iter(empty::<i8>()),
//                     "Unsaturation" => Series::from_iter(empty::<u8>()),
//                 }?.into_struct(PlSmallStr::EMPTY).into_series(),
//             ],
//         }?.into_struct(PlSmallStr::EMPTY),
//         "Value" => [
//             Some(1f64),
//             Some(2f64),
//             Some(3f64),
//             Some(4f64),
//             Some(5f64),
//             Some(6f64),
//         ],
//         "RetentionTime" => &[
//             10f64,
//             11f64,
//             11.25f64,
//             11.5f64,
//             11.75f64,
//             12f64,
//         ],
//     }?;
//     println!("data_frame: {}", data_frame);
//     println!("data_frame: {}", data_frame.unnest(["FattyAcid"])?);
//     let mut lazy_frame = data_frame.lazy();
//     lazy_frame = lazy_frame.with_columns([
//         col("FattyAcid")
//             .fatty_acid()
//             .fractional_chain_length(col("RetentionTime"), Default::default())
//             .alias("FCL0"),
//         col("FattyAcid")
//             .fatty_acid()
//             .fractional_chain_length(
//                 col("RetentionTime"),
//                 ChainLengthOptions::new().logarithmic(true),
//             )
//             .alias("FCL1"),
//         col("FattyAcid")
//             .fatty_acid()
//             .equivalent_chain_lengths(col("RetentionTime"), ChainLengthOptions::default())
//             .alias("ECL"),
//         col("FattyAcid")
//             .fatty_acid()
//             .equivalent_chain_lengths(
//                 col("RetentionTime"),
//                 ChainLengthOptions::new().logarithmic(true),
//             )
//             .alias("ECL1"),
//     ]);
//     // .map_saturated_or(lit(0), |saturated| {
//     //     (col("TimeMean")
//     //         - col("TimeMean")
//     //             .filter(col("FattyAcid").fatty_acids())
//     //             .forward_fill(None))
//     //         / (col("TimeMean")
//     //             .filter(col("FattyAcid").fatty_acids())
//     //             .backward_fill(None)
//     //             - col("TimeMean")
//     //                 .filter(col("FattyAcid").fatty_acids())
//     //                 .forward_fill(None))
//     // })
//     // .alias("Saturated")]);
//     // saturated(expr.clone()).backward_fill(None) - saturated(expr).forward_fill(None)
//     println!("lazy_frame: {}", lazy_frame.clone().collect()?);
//     Ok(())
// }

mod ecn;
mod equal;
mod fatty_acid;
mod mask;
