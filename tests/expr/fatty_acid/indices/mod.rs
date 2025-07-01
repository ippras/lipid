use super::*;
use std::num::NonZeroI8;

#[test]
fn monounsaturated() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID).fatty_acid().monounsaturated(col("Float"))])
        .collect()?;
    let monounsaturated = data_frame["Monounsaturated"].f64()?.get(0).unwrap();
    // 326
    assert_eq!(
        monounsaturated,
        13.0 + 14.0 + 17.0 + 18.0 + 29.0 + 30.0 + 40.0 + 48.0 + 56.0 + 61.0
    );
    Ok(())
}

#[test]
fn polyunsaturated() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID).fatty_acid().polyunsaturated(col("Float"))])
        .collect()?;
    let polyunsaturated = data_frame["Polyunsaturated"].f64()?.get(0).unwrap();
    // 888
    assert_eq!(
        polyunsaturated,
        19.0 + 20.0
            + 21.0
            + 22.0
            + 23.0
            + 24.0
            + 25.0
            + 26.0
            + 31.0
            + 32.0
            + 33.0
            + 34.0
            + 35.0
            + 36.0
            + 37.0
            + 41.0
            + 42.0
            + 43.0
            + 44.0
            + 45.0
            + 49.0
            + 50.0
            + 51.0
            + 52.0
            + 53.0
    );
    Ok(())
}

#[test]
fn polyunsaturated_3() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col("Float")
            .filter(
                col(FATTY_ACID).fatty_acid().is_polyunsaturated().and(
                    col(FATTY_ACID)
                        .fatty_acid()
                        .is_unsaturated(NonZeroI8::new(-3)),
                ),
            )
            .sum()
            .alias("Polyunsaturated-3")])
        .collect()?;
    let polyunsaturated_3 = data_frame["Polyunsaturated-3"].f64()?.get(0).unwrap();
    // 297
    assert_eq!(
        polyunsaturated_3,
        22.0 + 26.0 + 34.0 + 36.0 + 37.0 + 44.0 + 45.0 + 53.0
    );
    Ok(())
}

#[test]
fn polyunsaturated_6() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col("Float")
            .filter(
                col(FATTY_ACID).fatty_acid().is_polyunsaturated().and(
                    col(FATTY_ACID)
                        .fatty_acid()
                        .is_unsaturated(NonZeroI8::new(-6)),
                ),
            )
            .sum()
            .alias("Polyunsaturated-6")])
        .collect()?;
    let polyunsaturated_6 = data_frame["Polyunsaturated-6"].f64()?.get(0).unwrap();
    // 487
    assert_eq!(
        polyunsaturated_6,
        19.0 + 20.0 + 21.0 + 31.0 + 33.0 + 35.0 + 41.0 + 42.0 + 43.0 + 49.0 + 50.0 + 51.0 + 52.0
    );
    Ok(())
}

#[test]
fn trans() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID).fatty_acid().trans(col("Float"))])
        .collect()?;
    let trans = data_frame["Trans"].f64()?.get(0).unwrap();
    assert_eq!(trans, 14.0 + 18.0 + 21.0 + 23.0 + 24.0 + 25.0);
    Ok(())
}

#[test]
fn unsaturated() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID).fatty_acid().unsaturated(col("Float"), None)])
        .collect()?;
    let unsaturated = data_frame["Unsaturated"].f64()?.get(0).unwrap();
    // 1214
    assert_eq!(
        unsaturated,
        13.0 + 14.0
            + 17.0
            + 18.0
            + 19.0
            + 20.0
            + 21.0
            + 22.0
            + 23.0
            + 24.0
            + 25.0
            + 26.0
            + 29.0
            + 30.0
            + 31.0
            + 32.0
            + 33.0
            + 34.0
            + 35.0
            + 36.0
            + 37.0
            + 40.0
            + 41.0
            + 42.0
            + 43.0
            + 44.0
            + 45.0
            + 48.0
            + 49.0
            + 50.0
            + 51.0
            + 52.0
            + 53.0
            + 56.0
            + 61.0
    );
    Ok(())
}

#[test]
fn unsaturated_3() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID)
            .fatty_acid()
            .unsaturated(col("Float"), NonZeroI8::new(-3))])
        .collect()?;
    let unsaturated_3 = data_frame["Unsaturated-3"].f64()?.get(0).unwrap();
    // 297
    assert_eq!(
        unsaturated_3,
        22.0 + 26.0 + 34.0 + 36.0 + 37.0 + 44.0 + 45.0 + 53.0
    );
    Ok(())
}

#[test]
fn unsaturated_6() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID)
            .fatty_acid()
            .unsaturated(col("Float"), NonZeroI8::new(-6))])
        .collect()?;
    let unsaturated_6 = data_frame["Unsaturated-6"].f64()?.get(0).unwrap();
    // 487
    assert_eq!(
        unsaturated_6,
        19.0 + 20.0 + 21.0 + 31.0 + 33.0 + 35.0 + 41.0 + 42.0 + 43.0 + 49.0 + 50.0 + 51.0 + 52.0
    );
    Ok(())
}

#[test]
fn eicosapentaenoic_and_docosahexaenoic() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID)
            .fatty_acid()
            .eicosapentaenoic_and_docosahexaenoic(col("Float"))])
        .collect()?;
    let eicosapentaenoic_and_docosahexaenoic = data_frame["EicosapentaenoicAndDocosahexaenoic"]
        .f64()?
        .get(0)
        .unwrap();
    assert_eq!(eicosapentaenoic_and_docosahexaenoic, 37.0 + 45.0);
    Ok(())
}

#[test]
fn fish_lipid_quality() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID)
            .fatty_acid()
            .fish_lipid_quality(col("Float"))])
        .collect()?;
    let fish_lipid_quality = data_frame["FishLipidQuality"].f64()?.get(0).unwrap();
    assert_epsilon!(fish_lipid_quality, (37.0 + 45.0) / 2278.0);
    Ok(())
}

#[test]
fn health_promoting_index() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID)
            .fatty_acid()
            .health_promoting_index(col("Float"))])
        .collect()?;
    let health_promoting_index = data_frame["HealthPromotingIndex"].f64()?.get(0).unwrap();
    assert_epsilon!(health_promoting_index, 1214.0 / (8.0 + 4.0 * 10.0 + 12.0));
    Ok(())
}

#[test]
fn hypocholesterolemic_to_hypercholesterolemic() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID)
            .fatty_acid()
            .hypocholesterolemic_to_hypercholesterolemic(col("Float"))])
        .collect()?;
    let hypocholesterolemic_to_hypercholesterolemic =
        data_frame["HypocholesterolemicToHypercholesterolemic"]
            .f64()?
            .get(0)
            .unwrap();
    assert_epsilon!(
        hypocholesterolemic_to_hypercholesterolemic,
        (17.0 + 888.0) / (8.0 + 10.0 + 12.0)
    );
    Ok(())
}

#[test]
fn index_of_atherogenicity() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID)
            .fatty_acid()
            .index_of_atherogenicity(col("Float"))])
        .collect()?;
    let index_of_atherogenicity = data_frame["IndexOfAtherogenicity"].f64()?.get(0).unwrap();
    assert_epsilon!(index_of_atherogenicity, (8.0 + 4.0 * 10.0 + 12.0) / 1214.0);
    Ok(())
}

#[test]
fn index_of_thrombogenicity() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID)
            .fatty_acid()
            .index_of_thrombogenicity(col("Float"))])
        .collect()?;
    let index_of_thrombogenicity = data_frame["IndexOfThrombogenicity"].f64()?.get(0).unwrap();
    assert_epsilon!(
        index_of_thrombogenicity,
        (10.0 + 12.0 + 16.0) / (0.5 * 326.0 + 0.5 * 487.0 + 3.0 * 297.0 + 297.0 / 487.0)
    );
    Ok(())
}

#[test]
fn linoleic_to_alpha_linolenic() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID)
            .fatty_acid()
            .linoleic_to_alpha_linolenic(col("Float"))])
        .collect()?;
    let linoleic_to_alpha_linolenic = data_frame["LinoleicToAlphaLinolenic"]
        .f64()?
        .get(0)
        .unwrap();
    assert_epsilon!(linoleic_to_alpha_linolenic, 19.0 / 22.0);
    Ok(())
}

#[test]
fn unsaturation_index() -> PolarsResult<()> {
    let data_frame = fatty_acids_with_float()?
        .lazy()
        .select([col(FATTY_ACID)
            .fatty_acid()
            .unsaturation_index(col("Float"))])
        .collect()?;
    let unsaturation_index = data_frame["UnsaturationIndex"].f64()?.get(0).unwrap();
    assert_eq!(
        unsaturation_index,
        13.0 * 1.0
            + 14.0 * 1.0
            + 17.0 * 1.0
            + 18.0 * 1.0
            + 19.0 * 2.0
            + 20.0 * 3.0
            + 21.0 * 3.0
            + 22.0 * 3.0
            + 23.0 * 3.0
            + 24.0 * 3.0
            + 25.0 * 3.0
            + 26.0 * 4.0
            + 29.0 * 1.0
            + 30.0 * 1.0
            + 31.0 * 2.0
            + 32.0 * 3.0
            + 33.0 * 3.0
            + 34.0 * 3.0
            + 35.0 * 4.0
            + 36.0 * 4.0
            + 37.0 * 5.0
            + 40.0 * 1.0
            + 41.0 * 2.0
            + 42.0 * 3.0
            + 43.0 * 4.0
            + 44.0 * 5.0
            + 45.0 * 6.0
            + 48.0 * 1.0
            + 49.0 * 2.0
            + 50.0 * 3.0
            + 51.0 * 4.0
            + 52.0 * 5.0
            + 53.0 * 6.0
            + 56.0 * 1.0
            + 61.0 * 1.0
    );
    Ok(())
}
