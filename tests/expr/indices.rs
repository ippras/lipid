use super::*;
use std::num::NonZeroI8;

#[test]
fn unsaturated() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .unsaturated(col("Index").cast(DataType::UInt64))
            .alias("")])
        .collect()?;
    let unsaturated = data_frame[""]
        .as_materialized_series()
        .u64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    // 1214
    const EXPECTED: u64 = 13
        + 14
        + 17
        + 18
        + 19
        + 20
        + 21
        + 22
        + 23
        + 24
        + 25
        + 26
        + 29
        + 30
        + 31
        + 32
        + 33
        + 34
        + 35
        + 36
        + 37
        + 40
        + 41
        + 42
        + 43
        + 44
        + 45
        + 48
        + 49
        + 50
        + 51
        + 52
        + 53
        + 56
        + 61;
    assert_eq!(unsaturated, EXPECTED);
    Ok(())
}

#[test]
pub fn unsaturated_minus_3() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("Index")
            .cast(DataType::UInt64)
            .filter(
                col("FattyAcid")
                    .fatty_acid()
                    .is_unsaturated(NonZeroI8::new(-3)),
            )
            .sum()
            .alias("")])
        .collect()?;
    let unsaturated_minus_3 = data_frame[""]
        .as_materialized_series()
        .u64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    // 297
    const EXPECTED: u64 = 22 + 26 + 34 + 36 + 37 + 44 + 45 + 53;
    assert_eq!(unsaturated_minus_3, EXPECTED);
    Ok(())
}

#[test]
pub fn unsaturated_minus_6() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("Index")
            .cast(DataType::UInt64)
            .filter(
                col("FattyAcid")
                    .fatty_acid()
                    .is_unsaturated(NonZeroI8::new(-6)),
            )
            .sum()
            .alias("")])
        .collect()?;
    let unsaturated_minus_6 = data_frame[""]
        .as_materialized_series()
        .u64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    // 487
    const EXPECTED: u64 = 19 + 20 + 21 + 31 + 33 + 35 + 41 + 42 + 43 + 49 + 50 + 51 + 52;
    assert_eq!(unsaturated_minus_6, EXPECTED);
    Ok(())
}

#[test]
fn monounsaturated() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .monounsaturated(col("Index").cast(DataType::UInt64))
            .alias("")])
        .collect()?;
    let monounsaturated = data_frame[""]
        .as_materialized_series()
        .u64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    // 326
    const EXPECTED: u64 = 13 + 14 + 17 + 18 + 29 + 30 + 40 + 48 + 56 + 61;
    assert_eq!(monounsaturated, EXPECTED);
    Ok(())
}

#[test]
fn polyunsaturated() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .polyunsaturated(col("Index").cast(DataType::UInt64))
            .alias("")])
        .collect()?;
    let polyunsaturated = data_frame[""]
        .as_materialized_series()
        .u64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    // 888
    const EXPECTED: u64 = 19
        + 20
        + 21
        + 22
        + 23
        + 24
        + 25
        + 26
        + 31
        + 32
        + 33
        + 34
        + 35
        + 36
        + 37
        + 41
        + 42
        + 43
        + 44
        + 45
        + 49
        + 50
        + 51
        + 52
        + 53;
    assert_eq!(polyunsaturated, EXPECTED);
    Ok(())
}

#[test]
pub fn polyunsaturated_minus_3() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("Index")
            .cast(DataType::UInt64)
            .filter(
                col("FattyAcid").fatty_acid().is_polyunsaturated().and(
                    col("FattyAcid")
                        .fatty_acid()
                        .is_unsaturated(NonZeroI8::new(-3)),
                ),
            )
            .sum()
            .alias("")])
        .collect()?;
    let polyunsaturated_minus_3 = data_frame[""]
        .as_materialized_series()
        .u64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    // 297
    const EXPECTED: u64 = 22 + 26 + 34 + 36 + 37 + 44 + 45 + 53;
    assert_eq!(polyunsaturated_minus_3, EXPECTED);
    Ok(())
}

#[test]
pub fn polyunsaturated_minus_6() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("Index")
            .cast(DataType::UInt64)
            .filter(
                col("FattyAcid")
                    .fatty_acid()
                    .is_unsaturated(NonZeroI8::new(-6)),
            )
            .sum()
            .alias("")])
        .collect()?;
    let polyunsaturated_minus_6 = data_frame[""]
        .as_materialized_series()
        .u64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    // 487
    const EXPECTED: u64 = 19 + 20 + 21 + 31 + 33 + 35 + 41 + 42 + 43 + 49 + 50 + 51 + 52;
    assert_eq!(polyunsaturated_minus_6, EXPECTED);
    Ok(())
}

#[test]
pub fn index_of_atherogenicity() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .index_of_atherogenicity(col("Index").cast(DataType::Float64))
            .alias("")])
        .collect()?;
    let index_of_atherogenicity = data_frame[""]
        .as_materialized_series()
        .f64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    const EXPECTED: f64 = (8.0 + 4.0 * 10.0 + 12.0) / 1214.0;
    assert!(epsilon(index_of_atherogenicity, EXPECTED));
    Ok(())
}

#[test]
pub fn index_of_thrombogenicity() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .index_of_thrombogenicity(col("Index").cast(DataType::Float64))
            .alias("")])
        .collect()?;
    let index_of_thrombogenicity = data_frame[""]
        .as_materialized_series()
        .f64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    const EXPECTED: f64 =
        (10.0 + 12.0 + 16.0) / (0.5 * 326.0 + 0.5 * 487.0 + 3.0 * 297.0 + 297.0 / 487.0);
    assert!(epsilon(index_of_thrombogenicity, EXPECTED));
    Ok(())
}

#[test]
pub fn hypercholesterolemic_ratio() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .hypercholesterolemic_ratio(col("Index").cast(DataType::Float64))
            .alias("")])
        .collect()?;
    let hypercholesterolemic_ratio = data_frame[""]
        .as_materialized_series()
        .f64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    const EXPECTED: f64 = (17.0 + 888.0) / (8.0 + 10.0 + 12.0);
    assert!(epsilon(hypercholesterolemic_ratio, EXPECTED));
    Ok(())
}

#[test]
pub fn health_promoting_index() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .health_promoting_index(col("Index").cast(DataType::Float64))
            .alias("")])
        .collect()?;
    let health_promoting_index = data_frame[""]
        .as_materialized_series()
        .f64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    const EXPECTED: f64 = 1214.0 / (8.0 + 4.0 * 10.0 + 12.0);
    assert!(epsilon(health_promoting_index, EXPECTED));
    Ok(())
}

#[test]
fn unsaturation_index() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .unsaturation_index(col("Index").cast(DataType::UInt64))
            .alias("")])
        .collect()?;
    let unsaturation_index = data_frame[""]
        .as_materialized_series()
        .u64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    const EXPECTED: u64 = 13 * 1
        + 14 * 1
        + 17 * 1
        + 18 * 1
        + 19 * 2
        + 20 * 3
        + 21 * 3
        + 22 * 3
        + 23 * 3
        + 24 * 3
        + 25 * 3
        + 26 * 4
        + 29 * 1
        + 30 * 1
        + 31 * 2
        + 32 * 3
        + 33 * 3
        + 34 * 3
        + 35 * 4
        + 36 * 4
        + 37 * 5
        + 40 * 1
        + 41 * 2
        + 42 * 3
        + 43 * 4
        + 44 * 5
        + 45 * 6
        + 48 * 1
        + 49 * 2
        + 50 * 3
        + 51 * 4
        + 52 * 5
        + 53 * 6
        + 56 * 1
        + 61 * 1;
    assert_eq!(unsaturation_index, EXPECTED);
    Ok(())
}

#[test]
fn eicosapentaenoic_and_docosahexaenoic_sum() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .eicosapentaenoic_and_docosahexaenoic_sum(col("Index").cast(DataType::UInt64))
            .alias("")])
        .collect()?;
    let eicosapentaenoic_and_docosahexaenoic_sum = data_frame[""]
        .as_materialized_series()
        .u64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    const EXPECTED: u64 = 37 + 45;
    assert_eq!(eicosapentaenoic_and_docosahexaenoic_sum, EXPECTED);
    Ok(())
}

#[test]
fn fish_lipid_quality() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .fish_lipid_quality(col("Index").cast(DataType::Float64))
            .alias("")])
        .collect()?;
    let fish_lipid_quality = data_frame[""]
        .as_materialized_series()
        .f64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    const EXPECTED: f64 = (37.0 + 45.0) / 2346.0;
    assert!(epsilon(fish_lipid_quality, EXPECTED));
    Ok(())
}

#[test]
fn linoleic_to_alpha_linolenic() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .linoleic_to_alpha_linolenic(col("Index").cast(DataType::Float64))
            .alias("")])
        .collect()?;
    let linoleic_to_alpha_linolenic = data_frame[""]
        .as_materialized_series()
        .f64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    const EXPECTED: f64 = 19.0 / 22.0;
    assert!(epsilon(linoleic_to_alpha_linolenic, EXPECTED));
    Ok(())
}

#[test]
fn trans() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .trans(col("Index").cast(DataType::UInt64))
            .alias("")])
        .collect()?;
    let trans = data_frame[""]
        .as_materialized_series()
        .u64()?
        .into_iter()
        .next()
        .flatten()
        .unwrap();
    const EXPECTED: u64 = 14 + 18 + 21 + 23 + 24 + 25;
    assert_eq!(trans, EXPECTED);
    Ok(())
}
