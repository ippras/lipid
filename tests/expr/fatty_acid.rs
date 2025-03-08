use lipid::{fatty_acid::r#const::*, prelude::*};
use polars::prelude::*;
use std::sync::LazyLock;

const ALL: [&str; 10] = [S, D, DC, DT, T, TC, TT, U, UC, UT];

pub static SOURCE: LazyLock<DataFrame> = LazyLock::new(|| {
    (|| {
        df! {
            "FattyAcid" => [
                Some(Series::from_iter(C14U0).cast(&BOUND_DATA_TYPE).unwrap()),
                Some(Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE).unwrap()),
                Some(Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE).unwrap()),
                Some(Series::from_iter(ALL).cast(&BOUND_DATA_TYPE).unwrap()),
                Some(Series::from_iter([Some(S), None]).cast(&BOUND_DATA_TYPE).unwrap()),
                None,
            ],
        }
        .unwrap()
        .with_row_index("Index".into(), None)
    })()
    .unwrap()
});

#[test]
fn bounds() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().bounds().alias("Bounds")])
        .collect()
        .unwrap();
    let bounds = data_frame["Bounds"]
        .as_materialized_series()
        .u8()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        bounds,
        [Some(13), Some(17), Some(17), Some(10), Some(2), None],
    );
}

#[test]
fn is_saturated() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .is_saturated()
            .alias("IsSaturated")])
        .collect()
        .unwrap();
    let is_saturated = data_frame["IsSaturated"]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        is_saturated,
        [
            Some(true),
            Some(true),
            Some(false),
            Some(false),
            Some(false),
            None
        ],
    );
}

#[test]
fn is_unsaturated() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .is_unsaturated()
            .alias("IsUnsaturated")])
        .collect()
        .unwrap();
    let is_unsaturated = data_frame["IsUnsaturated"]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        is_unsaturated,
        [
            Some(false),
            Some(false),
            Some(true),
            Some(true),
            Some(false),
            None
        ],
    );
}

#[test]
fn saturated() {
    let data_frame_with_keep = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .saturated(false)
            .alias("SaturatedWithKeep")])
        .collect()
        .unwrap();
    let saturated_with_keep = data_frame_with_keep["SaturatedWithKeep"]
        .as_materialized_series()
        .fatty_acid()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        saturated_with_keep,
        [
            Some(Series::from_iter(C14U0)),
            Some(Series::from_iter(C18U0)),
            None,
            None,
            None,
            None,
        ],
    );
    let data_frame_without_keep = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .saturated(true)
            .alias("SaturatedWithoutKeep")])
        .collect()
        .unwrap();
    let saturated_without_keep = data_frame_without_keep["SaturatedWithoutKeep"]
        .as_materialized_series()
        .fatty_acid()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        saturated_without_keep,
        [
            Some(Series::from_iter(C14U0)),
            Some(Series::from_iter(C18U0)),
        ],
    );
}

#[test]
fn unsaturated() {
    let data_frame_with_keep = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .unsaturated(false)
            .alias("UnsaturatedWithKeep")])
        .collect()
        .unwrap();
    let unsaturated_with_keep = data_frame_with_keep["UnsaturatedWithKeep"]
        .as_materialized_series()
        .fatty_acid()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        unsaturated_with_keep,
        [
            None,
            None,
            Some(Series::from_iter(C18U2DC9DC12)),
            Some(Series::from_iter(ALL)),
            None,
            None,
        ]
    );
    let data_frame_without_keep = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .unsaturated(true)
            .alias("UnsaturatedWithoutKeep")])
        .collect()
        .unwrap();
    let unsaturated_without_keep = data_frame_without_keep["UnsaturatedWithoutKeep"]
        .as_materialized_series()
        .fatty_acid()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        unsaturated_without_keep,
        [
            Some(Series::from_iter(C18U2DC9DC12)),
            Some(Series::from_iter(ALL)),
        ],
    );
}

#[test]
fn unsaturation() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .unsaturation()
            .alias("Unsaturation")])
        .collect()
        .unwrap();
    let unsaturation = data_frame["Unsaturation"]
        .as_materialized_series()
        .u8()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        unsaturation,
        [Some(0), Some(0), Some(2), Some(9), Some(0), None],
    );
}

#[test]
fn carbons() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().carbons().alias("Carbons")])
        .collect()
        .unwrap();
    let carbons = data_frame["Carbons"]
        .as_materialized_series()
        .u8()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        carbons,
        [Some(14), Some(18), Some(18), Some(11), Some(3), None],
    );
}

#[test]
fn hydrogens() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().hydrogens().alias("Hydrogens")])
        .collect()
        .unwrap();
    let hydrogens = data_frame["Hydrogens"]
        .as_materialized_series()
        .u8()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        hydrogens,
        [Some(28), Some(36), Some(32), Some(4), Some(6), None],
    );
}

#[test]
fn equivalent_carbon_number() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .filter(col("Index").neq(lit(3)))
        .select([col("FattyAcid")
            .fatty_acid()
            .equivalent_carbon_number()
            .alias("EquivalentCarbonNumber")])
        .collect()
        .unwrap();
    let equivalent_carbon_number = data_frame["EquivalentCarbonNumber"]
        .as_materialized_series()
        .u8()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        equivalent_carbon_number,
        [Some(14), Some(18), Some(14), Some(3), None],
    );
}

// #[test]
// fn equivalent_chain_lengths() {
//     let retention_time = col("RetentionTime");
//     let options = Options::new();
//     let data_frame = df! {
//         "FattyAcid" => [
//             Some(Series::from_iter(C14U0).cast(&BOUND_DATA_TYPE).unwrap()),
//             Some(Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE).unwrap()),
//             Some(Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE).unwrap()),
//             Some(Series::from_iter(ALL).cast(&BOUND_DATA_TYPE).unwrap()),
//             Some(Series::from_iter([Some(S), None]).cast(&BOUND_DATA_TYPE).unwrap()),
//             None,
//         ],
//         "RetentionTime" => [
//             Some(10.0),
//             Some(20.0),
//             Some(30.0),
//             Some(40.0),
//             Some(50.0),
//             None,
//         ],
//     }
//     .unwrap();
//     let data_frame = data_frame
//         .clone()
//         .lazy()
//         .select([col("FattyAcid")
//             .fatty_acid()
//             .equivalent_chain_lengths(retention_time, options)
//             .alias("EquivalentChainLengths")])
//         .collect()
//         .unwrap();
//     let equivalent_chain_lengths = data_frame["EquivalentChainLengths"]
//         .as_materialized_series()
//         .f64()
//         .unwrap()
//         .into_iter()
//         .collect::<Vec<_>>();
//     assert_eq!(
//         equivalent_chain_lengths,
//         [Some(14.0), Some(18.0), Some(18.0), Some(11.0), Some(3.0), None],
//     );
// }

#[test]
fn temp() {
    println!("SOURCE: {SOURCE:?}");
    let data_frame = SOURCE
        .clone()
        .lazy()
        .filter(
            col("FattyAcid").fatty_acid().equal(
                lit(Scalar::new(
                    DataType::List(Box::new(BOUND_DATA_TYPE.clone())),
                    AnyValue::List(
                        Series::from_iter(C18U2DC9DC12)
                            .cast(&BOUND_DATA_TYPE)
                            .unwrap(),
                    ),
                ))
                .fatty_acid(),
            ),
        )
        // .select([col("FattyAcid")
        //     .filter(
        //         col("FattyAcid").fatty_acid().equal(
        //             lit(Scalar::new(
        //                 DataType::List(Box::new(BOUND_DATA_TYPE.clone())),
        //                 AnyValue::List(
        //                     Series::from_iter(C18U2DC9DC12)
        //                         .cast(&BOUND_DATA_TYPE)
        //                         .unwrap(),
        //                 ),
        //             ))
        //             .fatty_acid(),
        //         ),
        //     )
        //     .alias("Find")])
        .collect()
        .unwrap();
    println!("data_frame: {data_frame}");
}

// #[test]
// fn r#type()  {
//     let data_frame = SOURCE
//         .clone()
//         .lazy()
//         .select([col("FattyAcid").fatty_acid().r#type().alias("Type")])
//         .collect().unwrap();
//     let r#type = data_frame["Type"]
//         .as_materialized_series()
//         .categorical().unwrap()
//         .iter_str()
//         .collect::<Vec<_>>();
//     assert_eq!(r#type, [Some(S), Some(S), Some(U), Some(U), None, None]);
//
// }

// #[test]
// fn unsaturated()  {
//     let target = df! {
//         "Unsaturated" => &[
//             df! {
//                 "Index" => Series::new_empty(PlSmallStr::EMPTY, &IDX_DTYPE),
//                 "" => Series::new_empty(PlSmallStr::EMPTY, &BOUND_DATA_TYPE),
//             }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
//             df! {
//                 "Index" => Series::new_empty(PlSmallStr::EMPTY, &IDX_DTYPE),
//                 "" => Series::new_empty(PlSmallStr::EMPTY, &BOUND_DATA_TYPE),
//             }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
//             df! {
//                 "Index" => Series::from_iter([8, 11]).cast(&IDX_DTYPE).unwrap(),
//                 "" => Series::from_iter([DC, DC]).cast(&BOUND_DATA_TYPE).unwrap(),
//             }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
//             df! {
//                 "Index" => Series::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9]).cast(&IDX_DTYPE).unwrap(),
//                 "" => Series::from_iter([D, DC, DT, T, TC, TT, U, UC, UT]).cast(&BOUND_DATA_TYPE).unwrap(),
//             }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
//         ],
//     }.unwrap();
//     let data_frame = SOURCE
//         .clone()
//         .lazy()
//         .select([col("FattyAcid")
//             .fatty_acid()
//             .unsaturated(true)
//             .alias("Unsaturated")])
//         .collect().unwrap();
//     assert_eq!(data_frame, target);
//
// }
