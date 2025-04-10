use super::*;
use polars::prelude::*;

fn tag<const SN1: usize, const SN2: usize, const SN3: usize>(
    sn1: (&str, FattyAcid<SN1>),
    sn2: (&str, FattyAcid<SN2>),
    sn3: (&str, FattyAcid<SN3>),
) -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
            "StereospecificNumber1" => df! {
                "Label" => &[sn1.0],
                "FattyAcid" => &[Series::from_iter(sn1.1).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &[sn2.0],
                "FattyAcid" => &[Series::from_iter(sn2.1).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &[sn3.0],
                "FattyAcid" => &[Series::from_iter(sn3.1).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

mod chain_length;
#[cfg(feature = "mass")]
mod mass;
mod permutation;
