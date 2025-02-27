use lipid::{
    polars::bound::identifiers::{D, DC, DT, S, T, TC, TT},
    prelude::*,
};
use polars::prelude::*;
use std::sync::LazyLock;

const C14U0: [&str; 13] = [S, S, S, S, S, S, S, S, S, S, S, S, S];
const C18U0: [&str; 17] = [S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S];
const C18U2C9T12: [&str; 17] = [S, S, S, S, S, S, S, S, DC, S, S, DC, S, S, S, S, S];

const C: [&str; 7] = [S, D, DC, DT, T, TC, TT];

pub const SOURCE: LazyLock<PolarsResult<DataFrame>> = LazyLock::new(|| {
    df! {
        "Index" => [0, 1, 2, 3],
        "FattyAcid" => [
            Series::from_iter(C14U0).cast(&Bound::DATA_TYPE)?,
            Series::from_iter(C18U0).cast(&Bound::DATA_TYPE)?,
            Series::from_iter(C18U2C9T12).cast(&Bound::DATA_TYPE)?,
            Series::from_iter(C).cast(&Bound::DATA_TYPE)?,
        ],
    }
});

#[test]
fn bounds() -> PolarsResult<()> {
    let target = df! {
        "Bounds" => &[13, 17, 17, 7],
    }?;
    let data_frame = SOURCE
        .clone()?
        .lazy()
        .select([col("FattyAcid").fatty_acid().bounds().alias("Bounds")])
        .collect()?;
    assert_eq!(data_frame, target);
    Ok(())
}

#[test]
fn carbons() -> PolarsResult<()> {
    let target = df! {
        "Carbons" => &[14, 18, 18, 8],
    }?;
    let data_frame = SOURCE
        .clone()?
        .lazy()
        .select([col("FattyAcid").fatty_acid().carbons().alias("Carbons")])
        .collect()?;
    assert_eq!(data_frame, target);
    Ok(())
}

#[test]
fn is_saturated() -> PolarsResult<()> {
    let target = df! {
        "IsSaturated" => &[true, true, false, false],
    }?;
    let data_frame = SOURCE
        .clone()?
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .is_saturated()
            .alias("IsSaturated")])
        .collect()?;
    assert_eq!(data_frame, target);
    Ok(())
}

#[test]
fn is_unsaturated() -> PolarsResult<()> {
    let target = df! {
        "IsUnsaturated" => &[false, false, true, true],
    }?;
    let data_frame = SOURCE
        .clone()?
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .is_unsaturated()
            .alias("IsUnsaturated")])
        .collect()?;
    assert_eq!(data_frame, target);
    Ok(())
}

#[test]
fn saturated_or_null() -> PolarsResult<()> {
    let target = df! {
        "SaturatedOrNull" => &[Some(0), Some(1), None, None],
    }?;
    let data_frame = SOURCE
        .clone()?
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .saturated_or_null(col("Index"))
            .alias("SaturatedOrNull")])
        .collect()?;
    assert_eq!(data_frame, target);
    Ok(())
}

#[test]
fn r#type() -> PolarsResult<()> {
    use lipid::polars::r#type::identifiers::{S, U};

    let target = df! {
        "Type" => &[S, S, U, U],
    }?;
    let data_frame = SOURCE
        .clone()?
        .lazy()
        .select([col("FattyAcid").fatty_acid().r#type().alias("Type")])
        .collect()?;
    println!("data_frame: {data_frame}");
    assert_eq!(data_frame, target);
    Ok(())
}

#[test]
fn unsaturated_or_null() -> PolarsResult<()> {
    let target = df! {
        "UnsaturatedOrNull" => &[None, None, Some(2), Some(3)],
    }?;
    let data_frame = SOURCE
        .clone()?
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .unsaturated_or_null(col("Index"))
            .alias("UnsaturatedOrNull")])
        .collect()?;
    assert_eq!(data_frame, target);
    Ok(())
}

#[test]
fn unsaturated() -> PolarsResult<()> {
    let target = df! {
        "Unsaturated" => &[
            df! {
                "Index" => Series::new_empty(PlSmallStr::EMPTY, &IDX_DTYPE),
                "" => Series::new_empty(PlSmallStr::EMPTY, &Bound::DATA_TYPE),
            }?.into_struct(PlSmallStr::EMPTY).into_series(),
            df! {
                "Index" => Series::new_empty(PlSmallStr::EMPTY, &IDX_DTYPE),
                "" => Series::new_empty(PlSmallStr::EMPTY, &Bound::DATA_TYPE),
            }?.into_struct(PlSmallStr::EMPTY).into_series(),
            df! {
                "Index" => Series::from_iter([9, 12]).cast(&IDX_DTYPE)?,
                "" => Series::from_iter([DC, DC]).cast(&Bound::DATA_TYPE)?,
            }?.into_struct(PlSmallStr::EMPTY).into_series(),
            df! {
                "Index" => Series::from_iter([2, 3, 4, 5, 6, 7]).cast(&IDX_DTYPE)?,
                "" => Series::from_iter([D, DC, DT, T, TC, TT]).cast(&Bound::DATA_TYPE)?,
            }?.into_struct(PlSmallStr::EMPTY).into_series(),
        ],
    }?;
    let data_frame = SOURCE
        .clone()?
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .unsaturated()
            .alias("Unsaturated")])
        .collect()?;
    assert_eq!(data_frame, target);
    Ok(())
}

#[test]
fn unsaturated_function() -> PolarsResult<()> {
    // let target = df! {
    //     "Unsaturated" => &[
    //         df! {
    //             "Index" => Series::new_empty(PlSmallStr::EMPTY, &IDX_DTYPE),
    //             "" => Series::new_empty(PlSmallStr::EMPTY, &Bound::DATA_TYPE),
    //         }?.into_struct(PlSmallStr::EMPTY).into_series(),
    //         df! {
    //             "Index" => Series::new_empty(PlSmallStr::EMPTY, &IDX_DTYPE),
    //             "" => Series::new_empty(PlSmallStr::EMPTY, &Bound::DATA_TYPE),
    //         }?.into_struct(PlSmallStr::EMPTY).into_series(),
    //         df! {
    //             "Index" => Series::from_iter([9, 12]).cast(&IDX_DTYPE)?,
    //             "" => Series::from_iter([DC, DC]).cast(&Bound::DATA_TYPE)?,
    //         }?.into_struct(PlSmallStr::EMPTY).into_series(),
    //         df! {
    //             "Index" => Series::from_iter([2, 3, 4, 5, 6, 7]).cast(&IDX_DTYPE)?,
    //             "" => Series::from_iter([D, DC, DT, T, TC, TT]).cast(&Bound::DATA_TYPE)?,
    //         }?.into_struct(PlSmallStr::EMPTY).into_series(),
    //     ],
    // }?;
    let data_frame = SOURCE
        .clone()?
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .unsaturated()
            .alias("Unsaturated")])
        .collect()?;
    println!(
        "data_frame: {}",
        data_frame
            .explode(["Unsaturated"])
            .unwrap()
            .unnest(["Unsaturated"])
            .unwrap()
    );
    println!("data_frame: {data_frame}");
    // assert_eq!(data_frame, target);
    Ok(())
}

#[test]
fn methods() -> PolarsResult<()> {
    let source = df! {
        "FattyAcid" => [
            Series::from_iter(C14U0).cast(&Bound::DATA_TYPE)?,
            Series::from_iter(C18U0).cast(&Bound::DATA_TYPE)?,
            Series::from_iter(C18U2C9T12).cast(&Bound::DATA_TYPE)?,
            Series::from_iter(C).cast(&Bound::DATA_TYPE)?,
        ],
    }?;
    let target = df! {
        "Bounds" => &[13, 17, 17, 7],
        "IsSaturated" => &[true, true, false, false],
        "IsUnsaturated" => &[false, false, true, true],
        "Unsaturated" => &[
            df! {
                "Index" => Series::new_empty(PlSmallStr::EMPTY, &IDX_DTYPE),
                "" => Series::new_empty(PlSmallStr::EMPTY, &Bound::DATA_TYPE),
            }?.into_struct(PlSmallStr::EMPTY).into_series(),
            df! {
                "Index" => Series::new_empty(PlSmallStr::EMPTY, &IDX_DTYPE),
                "" => Series::new_empty(PlSmallStr::EMPTY, &Bound::DATA_TYPE),
            }?.into_struct(PlSmallStr::EMPTY).into_series(),
            df! {
                "Index" => Series::from_iter([9, 12]).cast(&IDX_DTYPE)?,
                "" => Series::from_iter([DC, DC]).cast(&Bound::DATA_TYPE)?,
            }?.into_struct(PlSmallStr::EMPTY).into_series(),
            df! {
                "Index" => Series::from_iter([2, 3, 4, 5, 6, 7]).cast(&IDX_DTYPE)?,
                "" => Series::from_iter([D, DC, DT, T, TC, TT]).cast(&Bound::DATA_TYPE)?,
            }?.into_struct(PlSmallStr::EMPTY).into_series(),
        ],
        "Carbons" => &[14, 18, 18, 8],
    }?;
    println!("data_frame: {}", source);
    let mut lazy_frame = source.lazy();
    lazy_frame = lazy_frame.select([
        col("FattyAcid").fatty_acid().bounds().alias("Bounds"),
        col("FattyAcid")
            .fatty_acid()
            .is_saturated()
            .alias("IsSaturated"),
        col("FattyAcid")
            .fatty_acid()
            .is_unsaturated()
            .alias("IsUnsaturated"),
        col("FattyAcid")
            .fatty_acid()
            .unsaturated()
            .alias("Unsaturated"),
        col("FattyAcid").fatty_acid().carbons().alias("Carbons"),
    ]);
    println!("lazy_frame: {}", lazy_frame.clone().collect()?);
    assert_eq!(lazy_frame.collect()?, target);
    Ok(())
}
