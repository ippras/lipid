#![feature(formatting_options)]
#![feature(custom_inner_attributes)]

use anyhow::Result;
use fatty_acid_macro::fatty_acid;
use lipid::prelude::*;
use polars::prelude::*;
use std::num::NonZeroI8;

#[test]
fn test1() -> Result<()> {
    let mut data_frame = df! {
        FATTY_ACID => Series::from_any_values_and_dtype(FATTY_ACID.into(), &[
            C16.clone(),
            C18.clone(),
            C18DC9.clone(),
            C18DC9DC12.clone(),
            C18DC9DC12DC15.clone(),
            fatty_acid!(C18 { -9 => DC })?,
            fatty_acid!(C18 { -9 => DC, -6 => DC })?,
            fatty_acid!(C18 { -9 => DC, -6 => DC, -3 => DC })?,
            fatty_acid!(C18 { 0 => U, 9 => U, 12 => T, 15 => D })?,
            C20.clone(),
        ], &data_type!(FATTY_ACID), true)?,
    }?
    .with_row_index("Index".into(), None)?;
    data_frame.rechunk_mut();
    println!("data_frame: {data_frame:?}");
    let lazy_frame = data_frame
        .lazy()
        .with_column(lit(1.0).alias("One"))
        .with_columns([
            col(FATTY_ACID).fatty_acid().is_saturated(),
            col(FATTY_ACID).fatty_acid().saturated(col("One")),
            col(FATTY_ACID).fatty_acid().unsaturated(col("One"), None),
            col(FATTY_ACID).fatty_acid().monounsaturated(col("One")),
            col(FATTY_ACID).fatty_acid().polyunsaturated(col("One")),
            col(FATTY_ACID)
                .fatty_acid()
                .polyunsaturated_to_saturated(col("One")),
            col(FATTY_ACID)
                .fatty_acid()
                .index_of_atherogenicity(col("One")),
            col(FATTY_ACID)
                .fatty_acid()
                .index_of_thrombogenicity(col("One")),
            col(FATTY_ACID)
                .fatty_acid()
                .hypocholesterolemic_to_hypercholesterolemic(col("One")),
            col(FATTY_ACID)
                .fatty_acid()
                .eicosapentaenoic_and_docosahexaenoic(col("One")),
            col(FATTY_ACID).fatty_acid().fish_lipid_quality(col("One")),
            col(FATTY_ACID)
                .fatty_acid()
                .linoleic_to_alpha_linolenic(col("One")),
            col(FATTY_ACID).fatty_acid().unsaturation_index(col("One")),
        ]);
    println!("lazy_frame: {:?}", lazy_frame.collect()?);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let data_frame = df! {
        FATTY_ACID => Series::from_any_values_and_dtype(FATTY_ACID.into(), &[
            fatty_acid!(C16 {})?,
            fatty_acid!(C18 {})?,
            fatty_acid!(C18 { 9 => DC })?,
            fatty_acid!(C18 { 9 => DC, 12 => DC  })?,
            fatty_acid!(C18 { 9 => DC, 12 => DC, 15 => DC })?,
            fatty_acid!(C18 { -9 => DC })?,
            fatty_acid!(C18 { -9 => DC, -6 => DC })?,
            fatty_acid!(C18 { -9 => DC, -6 => DC, -3 => DC })?,
        ], &data_type!(FATTY_ACID), true)?,
    }?;
    println!("data_frame: {data_frame:?}");
    Ok(())
}
