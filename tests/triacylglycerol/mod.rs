// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::special::fatty_acid::fatty_acid;
//     use anyhow::Result;

static MPS: LazyLock<DataFrame> = LazyLock::new(|| {
    || -> PolarsResult<DataFrame> {
        Ok(df! {
           "Triacylglycerol" => df! {
                "StereospecificNumber1" => &["M"],
                "StereospecificNumber2" => &["P"],
                "StereospecificNumber3" => &["S"],
            }?.into_struct(PlSmallStr::EMPTY),
        }?)
    }()
    .unwrap()
});

static MSP: LazyLock<DataFrame> = LazyLock::new(|| {
    || -> PolarsResult<DataFrame> {
        Ok(df! {
           "Triacylglycerol" => df! {
                "StereospecificNumber1" => &["M"],
                "StereospecificNumber2" => &["S"],
                "StereospecificNumber3" => &["P"],
            }?.into_struct(PlSmallStr::EMPTY),
        }?)
    }()
    .unwrap()
});

static PMS: LazyLock<DataFrame> = LazyLock::new(|| {
    || -> PolarsResult<DataFrame> {
        Ok(df! {
           "Triacylglycerol" => df! {
                "StereospecificNumber1" => &["P"],
                "StereospecificNumber2" => &["M"],
                "StereospecificNumber3" => &["S"],
            }?.into_struct(PlSmallStr::EMPTY),
        }?)
    }()
    .unwrap()
});

static PSM: LazyLock<DataFrame> = LazyLock::new(|| {
    || -> PolarsResult<DataFrame> {
        Ok(df! {
           "Triacylglycerol" => df! {
                "StereospecificNumber1" => &["P"],
                "StereospecificNumber2" => &["S"],
                "StereospecificNumber3" => &["M"],
            }?.into_struct(PlSmallStr::EMPTY),
        }?)
    }()
    .unwrap()
});

static SMP: LazyLock<DataFrame> = LazyLock::new(|| {
    || -> PolarsResult<DataFrame> {
        Ok(df! {
           "Triacylglycerol" => df! {
                "StereospecificNumber1" => &["S"],
                "StereospecificNumber2" => &["M"],
                "StereospecificNumber3" => &["P"],
            }?.into_struct(PlSmallStr::EMPTY),
        }?)
    }()
    .unwrap()
});

static SPM: LazyLock<DataFrame> = LazyLock::new(|| {
    || -> PolarsResult<DataFrame> {
        Ok(df! {
           "Triacylglycerol" => df! {
                "StereospecificNumber1" => &["S"],
                "StereospecificNumber2" => &["P"],
                "StereospecificNumber3" => &["M"],
            }?.into_struct(PlSmallStr::EMPTY),
        }?)
    }()
    .unwrap()
});

static C14: LazyLock<Series> = LazyLock::new(|| {
    || -> PolarsResult<_> {
        Ok(df! {
           "Carbons" => &[
                14u8,
            ],
            "Unsaturated" => &[
                df! {
                    "Index" => Series::from_iter(empty::<u8>()),
                    "Isomerism" => Series::from_iter(empty::<i8>()),
                    "Unsaturation" => Series::from_iter(empty::<u8>()),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
            ],
        }?
        .into_struct(PlSmallStr::EMPTY)
        .into_series())
    }()
    .unwrap()
});

static C16: LazyLock<Series> = LazyLock::new(|| {
    || -> PolarsResult<_> {
        Ok(df! {
           "Carbons" => &[
                16u8,
            ],
            "Unsaturated" => &[
                df! {
                    "Index" => Series::from_iter(empty::<u8>()),
                    "Isomerism" => Series::from_iter(empty::<i8>()),
                    "Unsaturation" => Series::from_iter(empty::<u8>()),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
            ],
        }?
        .into_struct(PlSmallStr::EMPTY)
        .into_series())
    }()
    .unwrap()
});

static C18: LazyLock<Series> = LazyLock::new(|| {
    || -> PolarsResult<_> {
        Ok(df! {
           "Carbons" => &[
                18u8,
            ],
            "Unsaturated" => &[
                df! {
                    "Index" => Series::from_iter(empty::<u8>()),
                    "Isomerism" => Series::from_iter(empty::<i8>()),
                    "Unsaturation" => Series::from_iter(empty::<u8>()),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
            ],
        }?
        .into_struct(PlSmallStr::EMPTY)
        .into_series())
    }()
    .unwrap()
});

static C16U1: LazyLock<Series> = LazyLock::new(|| {
    || -> PolarsResult<_> {
        Ok(df! {
           "Carbons" => &[
                16u8,
            ],
            "Unsaturated" => &[
                df! {
                   "Index" => Series::from_iter([9u8]),
                    "Isomerism" => Series::from_iter([1i8]),
                    "Unsaturation" => Series::from_iter([1u8]),
                }?.into_struct(PlSmallStr::EMPTY).into_series(),
            ],
        }?
        .into_struct(PlSmallStr::EMPTY)
        .into_series())
    }()
    .unwrap()
});

static C14C16U1C18: LazyLock<DataFrame> = LazyLock::new(|| {
    || -> PolarsResult<DataFrame> {
        Ok(df! {
           "Triacylglycerol" => df! {
                "StereospecificNumber1" => C14.clone(),
                "StereospecificNumber2" => C16U1.clone(),
                "StereospecificNumber3" => C18.clone(),
            }?.into_struct(PlSmallStr::EMPTY),
        }?)
    }()
    .unwrap()
});

static C14C18C16U1: LazyLock<DataFrame> = LazyLock::new(|| {
    || -> PolarsResult<DataFrame> {
        Ok(df! {
           "Triacylglycerol" => df! {
                "StereospecificNumber1" => C14.clone(),
                "StereospecificNumber2" => C18.clone(),
                "StereospecificNumber3" => C16U1.clone(),
            }?.into_struct(PlSmallStr::EMPTY),
        }?)
    }()
    .unwrap()
});

static C16U1C14C18: LazyLock<DataFrame> = LazyLock::new(|| {
    || -> PolarsResult<DataFrame> {
        Ok(df! {
           "Triacylglycerol" => df! {
                "StereospecificNumber1" => C16U1.clone(),
                "StereospecificNumber2" => C14.clone(),
                "StereospecificNumber3" => C18.clone(),
            }?.into_struct(PlSmallStr::EMPTY),
        }?)
    }()
    .unwrap()
});

static C16U1C18C14: LazyLock<DataFrame> = LazyLock::new(|| {
    || -> PolarsResult<DataFrame> {
        Ok(df! {
           "Triacylglycerol" => df! {
                "StereospecificNumber1" => C16U1.clone(),
                "StereospecificNumber2" => C18.clone(),
                "StereospecificNumber3" => C14.clone(),
            }?.into_struct(PlSmallStr::EMPTY),
        }?)
    }()
    .unwrap()
});

static C18C14C16U1: LazyLock<DataFrame> = LazyLock::new(|| {
    || -> PolarsResult<DataFrame> {
        Ok(df! {
           "Triacylglycerol" => df! {
                "StereospecificNumber1" => C18.clone(),
                "StereospecificNumber2" => C14.clone(),
                "StereospecificNumber3" => C16U1.clone(),
            }?.into_struct(PlSmallStr::EMPTY),
        }?)
    }()
    .unwrap()
});

static C18C16U1C14: LazyLock<DataFrame> = LazyLock::new(|| {
    || -> PolarsResult<DataFrame> {
        Ok(df! {
           "Triacylglycerol" => df! {
                "StereospecificNumber1" => C18.clone(),
                "StereospecificNumber2" => C16U1.clone(),
                "StereospecificNumber3" => C14.clone(),
            }?.into_struct(PlSmallStr::EMPTY),
        }?)
    }()
    .unwrap()
});

//     fn c14c18c16() -> PolarsResult<DataFrame> {
//         Ok(df! {
//            "TAG" => df! {
//                 "SN1" => df! {
//                     "FA" => df! {
//                         "Label" => &["M"],
//                         "Carbons" => &[14u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN2" => df! {
//                     "FA" => df! {
//                         "Label" => &["S"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN3" => df! {
//                     "FA" => df! {
//                         "Label" => &["P"],
//                         "Carbons" => &[16u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//             }?.into_struct(PlSmallStr::EMPTY),
//         }?)
//     }

//     fn c16c14c18() -> PolarsResult<DataFrame> {
//         Ok(df! {
//             "TAG" => df! {
//                 "SN1" => df! {
//                     "FA" => df! {
//                         "Label" => &["P"],
//                         "Carbons" => &[16u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN2" => df! {
//                     "FA" => df! {
//                         "Label" => &["M"],
//                         "Carbons" => &[14u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN3" => df! {
//                     "FA" => df! {
//                         "Label" => &["S"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//             }?.into_struct(PlSmallStr::EMPTY),
//         }?)
//     }

//     fn c16c18c14() -> PolarsResult<DataFrame> {
//         Ok(df! {
//             "TAG" => df! {
//                 "SN1" => df! {
//                     "FA" => df! {
//                         "Label" => &["P"],
//                         "Carbons" => &[16u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN2" => df! {
//                     "FA" => df! {
//                         "Label" => &["S"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN3" => df! {
//                     "FA" => df! {
//                         "Label" => &["M"],
//                         "Carbons" => &[14u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//             }?.into_struct(PlSmallStr::EMPTY),
//         }?)
//     }

//     fn c18c14c16() -> PolarsResult<DataFrame> {
//         Ok(df! {
//            "TAG" => df! {
//                 "SN1" => df! {
//                     "FA" => df! {
//                         "Label" => &["S"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN2" => df! {
//                     "FA" => df! {
//                         "Label" => &["M"],
//                         "Carbons" => &[14u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN3" => df! {
//                     "FA" => df! {
//                         "Label" => &["P"],
//                         "Carbons" => &[16u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//             }?.into_struct(PlSmallStr::EMPTY),
//         }?)
//     }

//     fn c18c16c14() -> PolarsResult<DataFrame> {
//         Ok(df! {
//            "TAG" => df! {
//                 "SN1" => df! {
//                     "FA" => df! {
//                         "Label" => &["S"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN2" => df! {
//                     "FA" => df! {
//                         "Label" => &["P"],
//                         "Carbons" => &[16u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN3" => df! {
//                     "FA" => df! {
//                         "Label" => &["M"],
//                         "Carbons" => &[14u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//             }?.into_struct(PlSmallStr::EMPTY),
//         }?)
//     }

//     fn su1u2() -> PolarsResult<DataFrame> {
//         Ok(df! {
//             "TAG" => df! {
//                 "SN1" => df! {
//                     "FA" => df! {
//                         "Label" => &["S"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN2" => df! {
//                     "FA" => df! {
//                         "Label" => &["O"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::from_iter([9])],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN3" => df! {
//                     "FA" => df! {
//                         "Label" => &["L"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::from_iter([9, 12])],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//             }?.into_struct(PlSmallStr::EMPTY),
//         }?)
//     }

//     fn su2u1() -> PolarsResult<DataFrame> {
//         Ok(df! {
//             "TAG" => df! {
//                 "SN1" => df! {
//                     "FA" => df! {
//                         "Label" => &["S"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN2" => df! {
//                     "FA" => df! {
//                         "Label" => &["L"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::from_iter([9, 12])],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN3" => df! {
//                     "FA" => df! {
//                         "Label" => &["O"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::from_iter([9])],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//             }?.into_struct(PlSmallStr::EMPTY),
//         }?)
//     }

//     fn u1su2() -> PolarsResult<DataFrame> {
//         Ok(df! {
//             "TAG" => df! {
//                 "SN1" => df! {
//                     "FA" => df! {
//                         "Label" => &["O"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::from_iter([9])],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN2" => df! {
//                     "FA" => df! {
//                         "Label" => &["S"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN3" => df! {
//                     "FA" => df! {
//                         "Label" => &["L"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::from_iter([9, 12])],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//             }?.into_struct(PlSmallStr::EMPTY),
//         }?)
//     }

//     fn u2su1() -> PolarsResult<DataFrame> {
//         Ok(df! {
//             "TAG" => df! {
//                 "SN1" => df! {
//                     "FA" => df! {
//                         "Label" => &["L"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::from_iter([9, 12])],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN2" => df! {
//                     "FA" => df! {
//                         "Label" => &["S"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN3" => df! {
//                     "FA" => df! {
//                         "Label" => &["O"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::from_iter([9])],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//             }?.into_struct(PlSmallStr::EMPTY),
//         }?)
//     }

//     fn u1u2s() -> PolarsResult<DataFrame> {
//         Ok(df! {
//             "TAG" => df! {
//                 "SN1" => df! {
//                     "FA" => df! {
//                         "Label" => &["O"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::from_iter([9])],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN2" => df! {
//                     "FA" => df! {
//                         "Label" => &["L"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::from_iter([9, 12])],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN3" => df! {
//                     "FA" => df! {
//                         "Label" => &["S"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//             }?.into_struct(PlSmallStr::EMPTY),
//         }?)
//     }

//     fn u2u1s() -> PolarsResult<DataFrame> {
//         Ok(df! {
//             "TAG" => df! {
//                 "SN1" => df! {
//                     "FA" => df! {
//                         "Label" => &["L"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::from_iter([9, 12])],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN2" => df! {
//                     "FA" => df! {
//                         "Label" => &["O"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::from_iter([9])],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//                 "SN3" => df! {
//                     "FA" => df! {
//                         "Label" => &["S"],
//                         "Carbons" => &[18u8],
//                         "Doubles" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                         "Triples" => &[Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8)],
//                     }?
//                     .into_struct(PlSmallStr::EMPTY),
//                 }?
//                 .into_struct(PlSmallStr::EMPTY),
//             }?.into_struct(PlSmallStr::EMPTY),
//         }?)
//     }

use lipid::prelude::*;
use polars::prelude::*;
use std::{convert::identity, iter::empty, sync::LazyLock};

// #[test]
// fn mc() -> PolarsResult<()> {
//     assert_eq!(
//         composition(c14c16c18()?, MC)?,
//         df! { "MC" => &[Series::from_iter([fatty_acid!(14).mass(), fatty_acid!(16).mass(), fatty_acid!(18).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c14c18c16()?, MC)?,
//         df! { "MC" => &[Series::from_iter([fatty_acid!(14).mass(), fatty_acid!(16).mass(), fatty_acid!(18).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c16c14c18()?, MC)?,
//         df! { "MC" => &[Series::from_iter([fatty_acid!(14).mass(), fatty_acid!(16).mass(), fatty_acid!(18).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c16c18c14()?, MC)?,
//         df! { "MC" => &[Series::from_iter([fatty_acid!(14).mass(), fatty_acid!(16).mass(), fatty_acid!(18).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c18c14c16()?, MC)?,
//         df! { "MC" => &[Series::from_iter([fatty_acid!(14).mass(), fatty_acid!(16).mass(), fatty_acid!(18).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c18c16c14()?, MC)?,
//         df! { "MC" => &[Series::from_iter([fatty_acid!(14).mass(), fatty_acid!(16).mass(), fatty_acid!(18).mass()])] }?,
//     );
//     Ok(())
// }

// #[test]
// fn pmc() -> PolarsResult<()> {
//     assert_eq!(
//         composition(c14c16c18()?, PMC)?,
//         df! { "PMC" => &[Series::from_iter([fatty_acid!(14).mass(), fatty_acid!(16).mass(), fatty_acid!(18).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c14c18c16()?, PMC)?,
//         df! { "PMC" => &[Series::from_iter([fatty_acid!(14).mass(), fatty_acid!(18).mass(), fatty_acid!(16).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c16c14c18()?, PMC)?,
//         df! { "PMC" => &[Series::from_iter([fatty_acid!(16).mass(), fatty_acid!(14).mass(), fatty_acid!(18).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c16c18c14()?, PMC)?,
//         df! { "PMC" => &[Series::from_iter([fatty_acid!(14).mass(), fatty_acid!(18).mass(), fatty_acid!(16).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c18c14c16()?, PMC)?,
//         df! { "PMC" => &[Series::from_iter([fatty_acid!(16).mass(), fatty_acid!(14).mass(), fatty_acid!(18).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c18c16c14()?, PMC)?,
//         df! { "PMC" => &[Series::from_iter([fatty_acid!(14).mass(), fatty_acid!(16).mass(), fatty_acid!(18).mass()])] }?,
//     );
//     Ok(())
// }

// #[test]
// fn smc() -> PolarsResult<()> {
//     assert_eq!(
//         composition(c14c16c18()?, SMC)?,
//         df! { "SMC" => &[Series::from_iter([fatty_acid!(14).mass(), fatty_acid!(16).mass(), fatty_acid!(18).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c14c18c16()?, SMC)?,
//         df! { "SMC" => &[Series::from_iter([fatty_acid!(14).mass(), fatty_acid!(18).mass(), fatty_acid!(16).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c16c14c18()?, SMC)?,
//         df! { "SMC" => &[Series::from_iter([fatty_acid!(16).mass(), fatty_acid!(14).mass(), fatty_acid!(18).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c16c18c14()?, SMC)?,
//         df! { "SMC" => &[Series::from_iter([fatty_acid!(16).mass(), fatty_acid!(18).mass(), fatty_acid!(14).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c18c14c16()?, SMC)?,
//         df! { "SMC" => &[Series::from_iter([fatty_acid!(18).mass(), fatty_acid!(14).mass(), fatty_acid!(16).mass()])] }?,
//     );
//     assert_eq!(
//         composition(c18c16c14()?, SMC)?,
//         df! { "SMC" => &[Series::from_iter([fatty_acid!(18).mass(), fatty_acid!(16).mass(), fatty_acid!(14).mass()])] }?,
//     );
//     Ok(())
// }

// #[test]
// fn nc() -> PolarsResult<()> {
//     assert_eq!(
//         composition(c14c16c18()?, NC)?,
//         df! { "NC" => &[Series::from_iter([fatty_acid!(14).ecn(), fatty_acid!(16).ecn(), fatty_acid!(18).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c14c18c16()?, NC)?,
//         df! { "NC" => &[Series::from_iter([fatty_acid!(14).ecn(), fatty_acid!(16).ecn(), fatty_acid!(18).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c16c14c18()?, NC)?,
//         df! { "NC" => &[Series::from_iter([fatty_acid!(14).ecn(), fatty_acid!(16).ecn(), fatty_acid!(18).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c16c18c14()?, NC)?,
//         df! { "NC" => &[Series::from_iter([fatty_acid!(14).ecn(), fatty_acid!(16).ecn(), fatty_acid!(18).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c18c14c16()?, NC)?,
//         df! { "NC" => &[Series::from_iter([fatty_acid!(14).ecn(), fatty_acid!(16).ecn(), fatty_acid!(18).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c18c16c14()?, NC)?,
//         df! { "NC" => &[Series::from_iter([fatty_acid!(14).ecn(), fatty_acid!(16).ecn(), fatty_acid!(18).ecn()])] }?,
//     );
//     Ok(())
// }

// #[test]
// fn pnc() -> PolarsResult<()> {
//     assert_eq!(
//         composition(c14c16c18()?, PNC)?,
//         df! { "PNC" => &[Series::from_iter([fatty_acid!(14).ecn(), fatty_acid!(16).ecn(), fatty_acid!(18).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c14c18c16()?, PNC)?,
//         df! { "PNC" => &[Series::from_iter([fatty_acid!(14).ecn(), fatty_acid!(18).ecn(), fatty_acid!(16).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c16c14c18()?, PNC)?,
//         df! { "PNC" => &[Series::from_iter([fatty_acid!(16).ecn(), fatty_acid!(14).ecn(), fatty_acid!(18).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c16c18c14()?, PNC)?,
//         df! { "PNC" => &[Series::from_iter([fatty_acid!(14).ecn(), fatty_acid!(18).ecn(), fatty_acid!(16).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c18c14c16()?, PNC)?,
//         df! { "PNC" => &[Series::from_iter([fatty_acid!(16).ecn(), fatty_acid!(14).ecn(), fatty_acid!(18).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c18c16c14()?, PNC)?,
//         df! { "PNC" => &[Series::from_iter([fatty_acid!(14).ecn(), fatty_acid!(16).ecn(), fatty_acid!(18).ecn()])] }?,
//     );
//     Ok(())
// }

// #[test]
// fn snc() -> PolarsResult<()> {
//     assert_eq!(
//         composition(c14c16c18()?, SNC)?,
//         df! { "SNC" => &[Series::from_iter([fatty_acid!(14).ecn(), fatty_acid!(16).ecn(), fatty_acid!(18).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c14c18c16()?, SNC)?,
//         df! { "SNC" => &[Series::from_iter([fatty_acid!(14).ecn(), fatty_acid!(18).ecn(), fatty_acid!(16).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c16c14c18()?, SNC)?,
//         df! { "SNC" => &[Series::from_iter([fatty_acid!(16).ecn(), fatty_acid!(14).ecn(), fatty_acid!(18).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c16c18c14()?, SNC)?,
//         df! { "SNC" => &[Series::from_iter([fatty_acid!(16).ecn(), fatty_acid!(18).ecn(), fatty_acid!(14).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c18c14c16()?, SNC)?,
//         df! { "SNC" => &[Series::from_iter([fatty_acid!(18).ecn(), fatty_acid!(14).ecn(), fatty_acid!(16).ecn()])] }?,
//     );
//     assert_eq!(
//         composition(c18c16c14()?, SNC)?,
//         df! { "SNC" => &[Series::from_iter([fatty_acid!(18).ecn(), fatty_acid!(16).ecn(), fatty_acid!(14).ecn()])] }?,
//     );
//     Ok(())
// }

#[cfg(test)]
mod species {
    use super::*;

    fn positional(data_frame: &DataFrame) -> PolarsResult<DataFrame> {
        data_frame
            .clone()
            .lazy()
            .select([col("Triacylglycerol")
                .triacylglycerol()
                .positional(identity, Default::default())])
            .collect()
    }

    fn non_stereospecific(data_frame: &DataFrame) -> PolarsResult<DataFrame> {
        data_frame
            .clone()
            .lazy()
            .select([col("Triacylglycerol")
                .triacylglycerol()
                .non_stereospecific(identity, Default::default())?])
            .collect()
    }

    #[test]
    fn sc() -> PolarsResult<()> {
        assert_eq!(non_stereospecific(&MPS)?, *MPS);
        assert_eq!(non_stereospecific(&MSP)?, *MPS);
        assert_eq!(non_stereospecific(&PMS)?, *MPS);
        assert_eq!(non_stereospecific(&PSM)?, *MPS);
        assert_eq!(non_stereospecific(&SMP)?, *MPS);
        assert_eq!(non_stereospecific(&SPM)?, *MPS);
        Ok(())
    }

    #[test]
    fn psc() -> PolarsResult<()> {
        assert_eq!(positional(&MPS)?, *MPS);
        assert_eq!(positional(&MSP)?, *MSP);
        assert_eq!(positional(&PMS)?, *PMS);
        assert_eq!(positional(&PSM)?, *MSP);
        assert_eq!(positional(&SMP)?, *PMS);
        assert_eq!(positional(&SPM)?, *MPS);
        Ok(())
    }
}

#[cfg(test)]
mod r#type {
    use super::*;

    fn positional(data_frame: &DataFrame) -> PolarsResult<DataFrame> {
        data_frame
            .clone()
            .lazy()
            .select([col("Triacylglycerol").triacylglycerol().positional(
                |expr| expr.fatty_acid().is_saturated().not(),
                Default::default(),
            )])
            .collect()
    }

    fn non_stereospecific(data_frame: &DataFrame) -> PolarsResult<DataFrame> {
        data_frame
            .clone()
            .lazy()
            .select([col("Triacylglycerol")
                .triacylglycerol()
                .non_stereospecific(
                    |expr| expr.fatty_acid().is_saturated().not(),
                    PermutationOptions::default().map(true),
                )?])
            .collect()
    }

    #[test]
    fn tc() -> PolarsResult<()> {
        // assert_eq!(non_stereospecific(&C14C16U1C18)?, *C14C18C16U1);
        // assert_eq!(non_stereospecific(&C14C18C16U1)?, *C14C18C16U1);
        // assert_eq!(non_stereospecific(&C16U1C14C18)?, *C14C18C16U1);
        // assert_eq!(non_stereospecific(&C16U1C18C14)?, *C14C18C16U1);
        // assert_eq!(non_stereospecific(&C18C14C16U1)?, *C14C18C16U1);
        // assert_eq!(non_stereospecific(&C18C16U1C14)?, *C14C18C16U1);
        Ok(())
    }

    // #[test]
    // fn ptc() -> PolarsResult<()> {
    //     assert_eq!(
    //         composition(su1u2()?, PTC)?,
    //         df! { "PTC" => &[Series::from_iter(["S", "U", "U"])] }?,
    //     );
    //     assert_eq!(
    //         composition(su2u1()?, PTC)?,
    //         df! { "PTC" => &[Series::from_iter(["S", "U", "U"])] }?,
    //     );
    //     assert_eq!(
    //         composition(u1su2()?, PTC)?,
    //         df! { "PTC" => &[Series::from_iter(["U", "S", "U"])] }?,
    //     );
    //     assert_eq!(
    //         composition(u2su1()?, PTC)?,
    //         df! { "PTC" => &[Series::from_iter(["U", "S", "U"])] }?,
    //     );
    //     assert_eq!(
    //         composition(u1u2s()?, PTC)?,
    //         df! { "PTC" => &[Series::from_iter(["S", "U", "U"])] }?,
    //     );
    //     assert_eq!(
    //         composition(u2u1s()?, PTC)?,
    //         df! { "PTC" => &[Series::from_iter(["S", "U", "U"])] }?,
    //     );
    //     Ok(())
    // }
}

// #[test]
// fn uc() -> PolarsResult<()> {
//     assert_eq!(
//         composition(su1u2()?, UC)?,
//         df! { "UC" => &[Series::from_iter([0, 1, 2])] }?,
//     );
//     assert_eq!(
//         composition(su2u1()?, UC)?,
//         df! { "UC" => &[Series::from_iter([0, 1, 2])] }?,
//     );
//     assert_eq!(
//         composition(u1su2()?, UC)?,
//         df! { "UC" => &[Series::from_iter([0, 1, 2])] }?,
//     );
//     assert_eq!(
//         composition(u2su1()?, UC)?,
//         df! { "UC" => &[Series::from_iter([0, 1, 2])] }?,
//     );
//     assert_eq!(
//         composition(u1u2s()?, UC)?,
//         df! { "UC" => &[Series::from_iter([0, 1, 2])] }?,
//     );
//     assert_eq!(
//         composition(u2u1s()?, UC)?,
//         df! { "UC" => &[Series::from_iter([0, 1, 2])] }?,
//     );
//     Ok(())
// }

// #[test]
// fn puc() -> PolarsResult<()> {
//     assert_eq!(
//         composition(su1u2()?, PUC)?,
//         df! { "PUC" => &[Series::from_iter([0, 1, 2])] }?,
//     );
//     assert_eq!(
//         composition(su2u1()?, PUC)?,
//         df! { "PUC" => &[Series::from_iter([0, 2, 1])] }?,
//     );
//     assert_eq!(
//         composition(u1su2()?, PUC)?,
//         df! { "PUC" => &[Series::from_iter([1, 0, 2])] }?,
//     );
//     assert_eq!(
//         composition(u2su1()?, PUC)?,
//         df! { "PUC" => &[Series::from_iter([1, 0, 2])] }?,
//     );
//     assert_eq!(
//         composition(u1u2s()?, PUC)?,
//         df! { "PUC" => &[Series::from_iter([0, 2, 1])] }?,
//     );
//     assert_eq!(
//         composition(u2u1s()?, PUC)?,
//         df! { "PUC" => &[Series::from_iter([0, 1, 2])] }?,
//     );
//     Ok(())
// }

// #[test]
// fn suc() -> PolarsResult<()> {
//     assert_eq!(
//         composition(su1u2()?, SUC)?,
//         df! { "SUC" => &[Series::from_iter([0, 1, 2])] }?,
//     );
//     assert_eq!(
//         composition(su2u1()?, SUC)?,
//         df! { "SUC" => &[Series::from_iter([0, 2, 1])] }?,
//     );
//     assert_eq!(
//         composition(u1su2()?, SUC)?,
//         df! { "SUC" => &[Series::from_iter([1, 0, 2])] }?,
//     );
//     assert_eq!(
//         composition(u2su1()?, SUC)?,
//         df! { "SUC" => &[Series::from_iter([2, 0, 1])] }?,
//     );
//     assert_eq!(
//         composition(u1u2s()?, SUC)?,
//         df! { "SUC" => &[Series::from_iter([1, 2, 0])] }?,
//     );
//     assert_eq!(
//         composition(u2u1s()?, SUC)?,
//         df! { "SUC" => &[Series::from_iter([2, 1, 0])] }?,
//     );
//     Ok(())
// }
