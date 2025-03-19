use lipid::prelude::*;
use polars::prelude::*;
use std::sync::LazyLock;

/// [DOI: 10.1007/s11746-014-2553-8](https://10.1007/s11746-014-2553-8)
static SOURCE: LazyLock<DataFrame> = LazyLock::new(|| {
    df! {
        "FattyAcid" => [
            Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C16U1DC9).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C18U3DC9DC12DC15).cast(&BOUND_DATA_TYPE).unwrap(),
        ],
        "E. bungeanus" => df! {
            "sn-1,2,3" => [28.1, 1.5, 10.2, 17.2, 41.7, 1.3],
            "sn-2" => [0.0, 0.0, 0.2, 11.2, 86.4, 2.1],
            "sn-1,3" => [40.0, 2.4, 14.0, 21.0, 21.5, 1.0],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
        "E. europaeus" => df! {
            "sn-1,2,3" => [23.2, 1.5, 6.8, 20.5, 45.6, 2.5],
            "sn-2" => [0.0, 0.0, 2.7, 12.5, 81.1, 3.7],
            "sn-1,3" => [28.6, 2.8, 6.9, 26.8, 32.8, 2.1],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
        "E. hamiltonianus" => df! {
            "sn-1,2,3" => [33.6, 2.5, 2.0, 16.4, 44.0, 1.5],
            "sn-2" => [0.0, 0.3, 2.1, 15.0, 79.8, 2.8],
            "sn-1,3" => [47.3, 3.7, 1.7, 17.9, 28.4, 1.0],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
        "E. phellomanus" => df! {
            "sn-1,2,3" => [31.9, 5.3, 1.3, 18.3, 41.3, 1.8],
            "sn-2" => [0.4, 0.0, 0.0, 10.8, 86.6, 2.1],
            "sn-1,3" => [44.7, 8.8, 1.9, 22.6, 20.5, 1.5],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
        "E. semiexsertus" => df! {
            "sn-1,2,3" => [28.8, 1.6, 0.6, 16.6, 51.3, 1.2],
            "sn-2" => [0.1, 0.0, 0.2, 7.3 , 90.5, 1.9],
            "sn-1,3" => [40.2, 2.5, 0.9, 21.8, 33.9, 0.8],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
        "E. sieboldianus" => df! {
            "sn-1,2,3" => [20.0, 2.1, 13.6, 25.5, 37.8, 1.0],
            "sn-2" => [0.1, 0.0, 7.0, 15.6, 75.4, 2.0],
            "sn-1,3" => [28.2, 3.3, 15.6, 31.5, 20.8, 0.6],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
        "E. alatus" => df! {
            "sn-1,2,3" => [24.6, 2.2, 0.5, 23.5, 46.4, 2.8],
            "sn-2" => [0.5, 0.1, 0.4, 18.4, 76.7, 3.9],
            "sn-1,3" => [34.1, 3.3, 0.4, 26.9, 33.0, 2.3],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
        "E. sacrosanctus" => df! {
            "sn-1,2,3" => [22.3, 2.0, 0.1, 28.1, 44.6, 2.9],
            "sn-2" => [0.5, 0., 0.1, 16.8, 77.5, 4.9],
            "sn-1,3" => [30.4, 2.8, 1.4, 34.3, 29.1, 2.0],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
        "E. pauciflorus" => df! {
            "sn-1,2,3" => [26.7, 5.4, 1.0, 39.4, 26.5, 1.1],
            "sn-2" => [0.5, 0.0, 0.0, 43.6, 54.0, 1.9],
            "sn-1,3" => [36.8, 8.7, 1.3, 38.7, 13.8, 0.7],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
        "E. latifolius" => df! {
            "sn-1,2,3" => [19.1, 2.0, 1.4, 56.5, 18.4, 2.6],
            "sn-2" => [0.2, 0.0, 1.3, 63.2, 30.7, 4.6],
            "sn-1,3" => [27.1, 3.1, 1.4, 54.2, 12.6, 1.7],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
        "E. macropterus" => df! {
            "sn-1,2,3" => [44.2, 2.7, 0.6, 19.8, 30.2, 2.6],
            "sn-2" => [0.2, 0.7, 0.8, 33.0, 60.0, 5.4],
            "sn-1,3" => [62.0, 3.8, 0.5, 14.8, 17.6, 1.4],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
        "E. maximowiczianus" => df! {
            "sn-1,2,3" => [34.4, 3.0, 0.7, 29.9, 30.9, 1.0],
            "sn-2" => [0.0, 0.0, 0.1, 40.3, 57.8, 1.7],
            "sn-1,3" => [49.5, 5.0, 1.0, 25.6, 18.2, 0.7],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
        "E. sachalinensis" => df! {
            "sn-1,2,3" => [33.7, 5.3, 0.4, 47.2, 11.9, 0.7],
            "sn-2" => [0.4, 0.0, 0.0, 68.2, 30.8, 0.6],
            "sn-1,3" => [47.1, 9.6, 0.6, 39.2, 2.9, 0.6],
        }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
    }
    .unwrap()
    .with_row_index("Index".into(), None)
    .unwrap()
});

// P St H O L Ln
#[test]
fn enrichment_factor() -> PolarsResult<()> {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([concat_list([all()
            .exclude(["FattyAcid", "Index"])
            .struct_()
            .field_by_name("sn-2")
            / all()
                .exclude(["FattyAcid", "Index"])
                .struct_()
                .field_by_name("sn-1,2,3")])?])
        .collect()?;
    let list = data_frame[0].list()?;
    let o = list
        .get_as_series(3)
        .unwrap()
        .round(2)?
        .f64()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        o,
        [
            Some(0.65),
            Some(0.61),
            Some(0.91),
            Some(0.59),
            Some(0.44),
            Some(0.61),
            Some(0.78),
            Some(0.60),
            Some(1.11),
            Some(1.12),
            Some(1.67),
            Some(1.35),
            Some(1.44), // ERROR: 1.45
        ],
    );
    let l = list
        .get_as_series(4)
        .unwrap()
        .round(2)?
        .f64()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        l,
        [
            Some(2.07),
            Some(1.78),
            Some(1.81),
            Some(2.10),
            Some(1.76),
            Some(1.99),
            Some(1.65),
            Some(1.74),
            Some(2.04),
            Some(1.67),
            Some(1.99),
            Some(1.87),
            Some(2.59),
        ],
    );
    Ok(())
}

#[test]
fn selectivity_factor() -> PolarsResult<()> {
    let calculate = |name| {
        col("FattyAcid")
            .fatty_acid()
            .selectivity_factor(
                col(name).struct_().field_by_name("sn-2"),
                col(name).struct_().field_by_name("sn-1,2,3"),
            )
            .round(2)
            .alias(name)
    };
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([
            calculate("E. bungeanus"),
            calculate("E. europaeus"),
            calculate("E. hamiltonianus"),
            calculate("E. phellomanus"),
            calculate("E. semiexsertus"),
            calculate("E. sieboldianus"),
            calculate("E. alatus"),
            calculate("E. sacrosanctus"),
            calculate("E. pauciflorus"),
            calculate("E. latifolius"),
            calculate("E. macropterus"),
            calculate("E. maximowiczianus"),
            calculate("E. sachalinensis"),
        ])
        .collect()?
        .slice(3, 2);
    assert_eq!(
        data_frame["E. bungeanus"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.46), Some(1.46)],
    );
    assert_eq!(
        data_frame["E. europaeus"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.46), Some(1.34)],
    );
    assert_eq!(
        data_frame["E. hamiltonianus"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.59), Some(1.16)], // ERROR: 0.58
    );
    assert_eq!(
        data_frame["E. phellomanus"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.37), Some(1.32)],
    );
    assert_eq!(
        data_frame["E. semiexsertus"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.31), Some(1.23)], // ERROR: 1.22
    );
    assert_eq!(
        data_frame["E. sieboldianus"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.48), Some(1.55)],
    );
    assert_eq!(
        data_frame["E. alatus"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.58), Some(1.22)], // ERROR: 0.57, ERROR: 1.21
    );
    assert_eq!(
        data_frame["E. sacrosanctus"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.46), Some(1.32)], // ERROR: 1.33
    );
    assert_eq!(
        data_frame["E. pauciflorus"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.76), Some(1.39)],
    );
    assert_eq!(
        data_frame["E. latifolius"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.88), Some(1.32)], // ERROR: 0.89, ERROR: 1.33
    );
    assert_eq!(
        data_frame["E. macropterus"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.89), Some(1.07)], // ERROR: 1.06
    );
    assert_eq!(
        data_frame["E. maximowiczianus"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.84), Some(1.17)],
    );
    assert_eq!(
        data_frame["E. sachalinensis"]
            .f64()?
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.87), Some(1.56)], // ERROR: 0.89, ERROR: 1.59
    );
    Ok(())
}
