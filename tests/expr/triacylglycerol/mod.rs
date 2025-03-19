use lipid::prelude::*;
use polars::prelude::*;

fn c14u0c16u0c18u0() -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
            "StereospecificNumber1" => df! {
                "Label" => &["M"],
                "FattyAcid" => &[Series::from_iter(C14U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &["P"],
                "FattyAcid" => &[Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &["S"],
                "FattyAcid" => &[Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

fn c14u0c18u0c16u0() -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
            "StereospecificNumber1" => df! {
                "Label" => &["M"],
                "FattyAcid" => &[Series::from_iter(C14U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &["S"],
                "FattyAcid" => &[Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &["P"],
                "FattyAcid" => &[Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

fn c16u0c14u0c18u0() -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
            "StereospecificNumber1" => df! {
                "Label" => &["P"],
                "FattyAcid" => &[Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &["M"],
                "FattyAcid" => &[Series::from_iter(C14U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &["S"],
                "FattyAcid" => &[Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

fn c16u0c18u0c14u0() -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
            "StereospecificNumber1" => df! {
                "Label" => &["P"],
                "FattyAcid" => &[Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &["S"],
                "FattyAcid" => &[Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &["M"],
                "FattyAcid" => &[Series::from_iter(C14U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

fn c18u0c14u0c16u0() -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
           "StereospecificNumber1" => df! {
                "Label" => &["S"],
                "FattyAcid" => &[Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE)?],
           }?
           .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &["M"],
                "FattyAcid" => &[Series::from_iter(C14U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &["P"],
                "FattyAcid" => &[Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

fn c18u0c16u0c14u0() -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
            "StereospecificNumber1" => df! {
                "Label" => &["S"],
                "FattyAcid" => &[Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &["P"],
                "FattyAcid" => &[Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &["M"],
                "FattyAcid" => &[Series::from_iter(C14U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

fn c18u0c18u1dc9c18u2dc9dc12() -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
            "StereospecificNumber1" => df! {
                "Label" => &["S"],
                "FattyAcid" => &[Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &["O"],
                "FattyAcid" => &[Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &["L"],
                "FattyAcid" => &[Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

fn c18u0c18u2dc9dc12c18u1dc9() -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
            "StereospecificNumber1" => df! {
                "Label" => &["S"],
                "FattyAcid" => &[Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &["L"],
                "FattyAcid" => &[Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &["O"],
                "FattyAcid" => &[Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

fn c18u1dc9c18u0c18u2dc9dc12() -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
            "StereospecificNumber1" => df! {
                "Label" => &["O"],
                "FattyAcid" => &[Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &["S"],
                "FattyAcid" => &[Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &["L"],
                "FattyAcid" => &[Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

fn c18u1dc9c18u2dc9dc12c18u0() -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
            "StereospecificNumber1" => df! {
                "Label" => &["O"],
                "FattyAcid" => &[Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &["L"],
                "FattyAcid" => &[Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &["S"],
                "FattyAcid" => &[Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

fn c18u2dc9dc12c18u0c18u1dc9() -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
            "StereospecificNumber1" => df! {
                "Label" => &["L"],
                "FattyAcid" => &[Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &["S"],
                "FattyAcid" => &[Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &["O"],
                "FattyAcid" => &[Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

fn c18u2dc9dc12c18u1dc9c18u0() -> PolarsResult<DataFrame> {
    Ok(df! {
       "Triacylglycerol" => df! {
            "StereospecificNumber1" => df! {
                "Label" => &["L"],
                "FattyAcid" => &[Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber2" => df! {
                "Label" => &["O"],
                "FattyAcid" => &[Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
            "StereospecificNumber3" => df! {
                "Label" => &["S"],
                "FattyAcid" => &[Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE)?],
            }?
            .into_struct(PlSmallStr::EMPTY),
        }?.into_struct(PlSmallStr::EMPTY),
    }?)
}

#[cfg(feature = "mass")]
mod mass;
mod permutation;
