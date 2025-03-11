// |E. bungeanus      ||sn-2|||||sn-1,3||||||
// |E. europaeus      ||sn-2|||||sn-1,3||||||
// |E. hamiltonianus  ||sn-2|||||sn-1,3||||||
// |E. phellomanus    ||sn-2|||||sn-1,3||||||
// |E. semiexsertus   ||sn-2|||||sn-1,3||||||
// |E. sieboldianus   ||sn-2|||||sn-1,3||||||
// |E. alatus         ||sn-2|||||sn-1,3||||||
// |E. sacrosantcus   ||sn-2|||||sn-1,3||||||
// |E. pauciflorus    ||sn-2|||||sn-1,3||||||
// |E. latifolius     ||sn-2|||||sn-1,3||||||
// |E. macropterus    ||sn-2|||||sn-1,3||||||
// |E. maximowiczianus||sn-2|||||sn-1,3||||||
// |E. sachalinensis  ||sn-2|||||sn-1,3||||||

use lipid::prelude::*;
use polars::prelude::*;
use std::sync::LazyLock;

/// [DOI: 10.1007/s11746-014-2553-8](https://10.1007/s11746-014-2553-8)
// pub static SOURCE: LazyLock<DataFrame> = LazyLock::new(|| {
//     df! {
//             "FattyAcid" => [
//                 Some(Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE).unwrap()),
//                 Some(Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE).unwrap()),
//                 Some(Series::from_iter(C16U1DC9).cast(&BOUND_DATA_TYPE).unwrap()),
//                 Some(Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE).unwrap()),
//                 Some(Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE).unwrap()),
//                 Some(Series::from_iter(C18U3DC9DC12DC15).cast(&BOUND_DATA_TYPE).unwrap()),
//             ],
//             "sn-1,2,3" => [
//                 Some(Series::from_iter([28.1, 23.2, 33.6, 31.9, 28.8, 20.0, 24.6, 22.3, 26.7, 19.1, 44.2, 34.4, 33.7])),
//                 Some(Series::from_iter([1.5, 1.5, 2.5, 5.3, 1.6, 2.1, 2.2, 2.0, 5.4, 2.0, 2.7, 3.0, 5.3])),
//                 Some(Series::from_iter([10.2, 6.8, 2.0, 1.3, 0.6, 13.6, 0.5, 0.1, 1.0, 1.4, 0.6, 0.7, 0.4])),
//                 Some(Series::from_iter([17.2, 20.5, 16.4, 18.3, 16.6, 25.5, 23.5, 28.1, 39.4, 56.5, 19.8, 29.9, 47.2])),
//                 Some(Series::from_iter([41.7, 45.6, 44.0, 41.3, 51.3, 37.8, 46.4, 44.6, 26.5, 18.4, 30.2, 30.9, 11.9])),
//                 Some(Series::from_iter([1.3, 2.5, 1.5, 1.8, 1.2, 1.0, 2.8, 2.9, 1.1, 2.6, 2.6, 1.0, 0.7])),
//             ],
//             "sn-2" => [
//                 Some(Series::from_iter([0.0, 0.0, 0.0, 0.4, 0.1, 0.1, 0.5, 0.5, 0.5, 0.2, 0.2, 0.0, 0.4])),
//                 Some(Series::from_iter([0.0, 0.0, 0.3, 0.0, 0.0, 0.0, 0.1, 0.3, 0.0, 0.0, 0.7, 0.0, 0.0])),
//                 Some(Series::from_iter([0.2, 2.7, 2.1, 0.0, 0.2, 7.0, 0.4, 0.1, 0.0, 1.3, 0.8, 0.1, 0.0])),
//                 Some(Series::from_iter([11.2, 12.5, 15.0, 10.8, 7.3, 15.6, 18.4, 16.8, 43.6, 63.2, 33.0, 40.3, 68.2])),
//                 Some(Series::from_iter([86.4, 81.1, 79.8, 86.6, 90.5, 75.4, 76.7, 77.5, 54.0, 30.7, 60.0, 57.8, 30.8])),
//                 Some(Series::from_iter([2.1, 3.7, 2.8, 2.1, 1.9, 2.0, 3.9, 4.9, 1.9, 4.6, 5.4, 1.7, 0.6])),
//             ],
//             "sn-1,3" => [
//                 Some(Series::from_iter([40.0, 28.6, 47.3, 44.7, 40.2, 28.2, 34.1, 30.4, 36.8, 27.1, 62.0, 49.5, 47.1])),
//                 Some(Series::from_iter([2.4, 2.8, 3.7, 8.8, 2.5, 3.3, 3.3, 2.8, 8.7, 3.1, 3.8, 5.0, 9.6])),
//                 Some(Series::from_iter([14.0, 6.9, 1.7, 1.9, 0.9, 15.6, 0.4, 1.4, 1.3, 1.4, 0.5, 1.0, 0.6])),
//                 Some(Series::from_iter([21.0, 26.8, 17.9, 22.6, 21.8, 31.5, 26.9, 34.3, 38.7, 54.2, 14.8, 25.6, 39.2])),
//                 Some(Series::from_iter([21.5, 32.8, 28.4, 20.5, 33.9, 20.8, 33.0, 29.1, 13.8, 12.6, 17.6, 18.2, 2.9])),
//                 Some(Series::from_iter([1.0, 2.1, 1.0, 1.5, 0.8, 0.6, 2.3, 2.0, 0.7, 1.7, 1.4, 0.7, 0.6])),
//             ],
//         }
//     .unwrap()
//     .with_row_index("Index".into(), None)
//     .unwrap()
// });

// pub static SOURCE: LazyLock<DataFrame> = LazyLock::new(|| {
//     df! {
//         "FattyAcid" => [
//             Some(Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE).unwrap()),
//             Some(Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE).unwrap()),
//             Some(Series::from_iter(C16U1DC9).cast(&BOUND_DATA_TYPE).unwrap()),
//             Some(Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE).unwrap()),
//             Some(Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE).unwrap()),
//             Some(Series::from_iter(C18U3DC9DC12DC15).cast(&BOUND_DATA_TYPE).unwrap()),
//         ],
//         "sn-1,2,3" => [
//             28.1,
//             1.5,
//             10.2,
//             17.2,
//             41.7,
//             1.3,
//         ],
//         "sn-2" => [
//             0.0,
//             0.0,
//             0.2,
//             11.2,
//             86.4,
//             2.1,
//         ],
//         "sn-1,3" => [
//             40.0,
//             2.4,
//             14.0,
//             21.0,
//             21.5,
//             1.0,
//         ],
//     }
//     .unwrap()
//     .with_row_index("Index".into(), None)
//     .unwrap()
// });

// pub static SOURCE: LazyLock<DataFrame> = LazyLock::new(|| {
//     df! {
//         "FattyAcid" => [
//             Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE).unwrap(),
//             Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE).unwrap(),
//             Series::from_iter(C16U1DC9).cast(&BOUND_DATA_TYPE).unwrap(),
//             Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE).unwrap(),
//             Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE).unwrap(),
//             Series::from_iter(C18U3DC9DC12DC15).cast(&BOUND_DATA_TYPE).unwrap(),
//         ],
//         "P" => df! {
//             "sn-1,2,3" => [28.1, 23.2, 33.6, 31.9, 28.8, 20.0, 24.6, 22.3, 26.7, 19.1, 44.2, 34.4, 33.7],
//             "sn-2" => [0.0, 0.0, 0.0, 0.4, 0.1, 0.1, 0.5, 0.5, 0.5, 0.2, 0.2, 0.0, 0.4],
//             "sn-1,3" => [40.0, 28.6, 47.3, 44.7, 40.2, 28.2, 34.1, 30.4, 36.8, 27.1, 62.0, 49.5, 47.1],
//         }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
//         "St" => df! {
//             "sn-1,2,3" => [1.5, 1.5, 2.5, 5.3, 1.6, 2.1, 2.2, 2.0, 5.4, 2.0, 2.7, 3.0, 5.3],
//             "sn-2" => [0.0, 0.0, 0.3, 0.0, 0.0, 0.0, 0.1, 0.3, 0.0, 0.0, 0.7, 0.0, 0.0],
//             "sn-1,3" => [2.4, 2.8, 3.7, 8.8, 2.5, 3.3, 3.3, 2.8, 8.7, 3.1, 3.8, 5.0, 9.6],
//         }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
//         "H" => df! {
//             "sn-1,2,3" => [10.2, 6.8, 2.0, 1.3, 0.6, 13.6, 0.5, 0.1, 1.0, 1.4, 0.6, 0.7, 0.4],
//             "sn-2" => [0.2, 2.7, 2.1, 0.0, 0.2, 7.0, 0.4, 0.1, 0.0, 1.3, 0.8, 0.1, 0.0],
//             "sn-1,3" => [14.0, 6.9, 1.7, 1.9, 0.9, 15.6, 0.4, 1.4, 1.3, 1.4, 0.5, 1.0, 0.6],
//         }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
//         "O" => df! {
//             "sn-1,2,3" => [17.2, 20.5, 16.4, 18.3, 16.6, 25.5, 23.5, 28.1, 39.4, 56.5, 19.8, 29.9, 47.2],
//             "sn-2" => [11.2, 12.5, 15.0, 10.8, 7.3, 15.6, 18.4, 16.8, 43.6, 63.2, 33.0, 40.3, 68.2],
//             "sn-1,3" => [21.0, 26.8, 17.9, 22.6, 21.8, 31.5, 26.9, 34.3, 38.7, 54.2, 14.8, 25.6, 39.2],
//         }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
//         "L" => df! {
//             "sn-1,2,3" => [41.7, 45.6, 44.0, 41.3, 51.3, 37.8, 46.4, 44.6, 26.5, 18.4, 30.2, 30.9, 11.9],
//             "sn-2" => [86.4, 81.1, 79.8, 86.6, 90.5, 75.4, 76.7, 77.5, 54.0, 30.7, 60.0, 57.8, 30.8],
//             "sn-1,3" => [21.5, 32.8, 28.4, 20.5, 33.9, 20.8, 33.0, 29.1, 13.8, 12.6, 17.6, 18.2, 2.9],
//         }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
//         "Ln" => df! {
//             "sn-1,2,3" => [1.3, 2.5, 1.5, 1.8, 1.2, 1.0, 2.8, 2.9, 1.1, 2.6, 2.6, 1.0, 0.7],
//             "sn-2" => [2.1, 3.7, 2.8, 2.1, 1.9, 2.0, 3.9, 4.9, 1.9, 4.6, 5.4, 1.7, 0.6],
//             "sn-1,3" => [1.0, 2.1, 1.0, 1.5, 0.8, 0.6, 2.3, 2.0, 0.7, 1.7, 1.4, 0.7, 0.6],
//         }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
//     }
//     .unwrap()
//     .with_row_index("Index".into(), None)
//     .unwrap()
// });
pub static SOURCE: LazyLock<DataFrame> = LazyLock::new(|| {
    df! {
        "FattyAcid" => [
            Series::from_iter(C16U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C18U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C16U1DC9).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C18U1DC9).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C18U2DC9DC12).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C18U3DC9DC12DC15).cast(&BOUND_DATA_TYPE).unwrap(),
        ],
        // "E. bungeanus" => df! {
        //     "sn-1,2,3" => [28.1, 1.5, 10.2, 17.2, 41.7, 1.3],
        //     "sn-2" => [0.0, 0.0, 0.0, 0.4, 0.1, 0.1, 0.5, 0.5, 0.5, 0.2, 0.2, 0.0, 0.4],
        //     "sn-1,3" => [40.0, 28.6, 47.3, 44.7, 40.2, 28.2, 34.1, 30.4, 36.8, 27.1, 62.0, 49.5, 47.1],
        // }.unwrap().into_struct(PlSmallStr::EMPTY).into_series(),
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
fn enrichment_factor() {
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
                .field_by_name("sn-1,2,3")])
        .unwrap()])
        // .select([
        //     all()
        //         .exclude(["FattyAcid", "Index"])
        //         .struct_()
        //         .field_by_name("sn-2")
        //         / all()
        //             .exclude(["FattyAcid", "Index"])
        //             .struct_()
        //             .field_by_name("sn-1,2,3")
        //     .alias("EnrichmentFactor"),
        //     // FattyAcidExpr::enrichment_factor(
        //     //     expr.clone().struct_().field_by_name("sn-2"),
        //     //     expr.clone()
        //     //         .struct_()
        //     //         .field_by_name("sn-1,2,3")
        //     //         .alias(expr.name().),
        //     // )
        //     // .round(2),
        // ])
        .collect()
        .unwrap();
    println!("data_frame: {data_frame}");
    let list = data_frame[0].list().unwrap();
    let o = list
        .get_as_series(3)
        .unwrap()
        .round(2)
        .unwrap()
        .f64()
        .unwrap()
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
        .round(2)
        .unwrap()
        .f64()
        .unwrap()
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
}

#[test]
pub fn selectivity_factor() {
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
        .collect()
        .unwrap();
    let data_frame = data_frame.slice(3, 2);
    println!("data_frame: {data_frame}");
    assert_eq!(
        data_frame["E. bungeanus"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.46), Some(1.46)],
    );
    assert_eq!(
        data_frame["E. europaeus"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.46), Some(1.34)],
    );
    assert_eq!(
        data_frame["E. hamiltonianus"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.59), Some(1.16)], // ERROR: 0.58
    );
    assert_eq!(
        data_frame["E. phellomanus"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.37), Some(1.32)],
    );
    assert_eq!(
        data_frame["E. semiexsertus"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.31), Some(1.23)], // ERROR: 1.22
    );
    assert_eq!(
        data_frame["E. sieboldianus"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.48), Some(1.55)],
    );
    assert_eq!(
        data_frame["E. alatus"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.58), Some(1.22)], // ERROR: 0.57, ERROR: 1.21
    );
    assert_eq!(
        data_frame["E. sacrosanctus"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.46), Some(1.32)], // ERROR: 1.33
    );
    assert_eq!(
        data_frame["E. pauciflorus"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.76), Some(1.39)],
    );
    assert_eq!(
        data_frame["E. latifolius"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.88), Some(1.32)], // ERROR: 0.89, ERROR: 1.33
    );
    assert_eq!(
        data_frame["E. macropterus"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.89), Some(1.07)], // ERROR: 1.06
    );
    assert_eq!(
        data_frame["E. maximowiczianus"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.84), Some(1.17)],
    );
    assert_eq!(
        data_frame["E. sachalinensis"]
            .f64()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.87), Some(1.56)], // ERROR: 0.89, ERROR: 1.59
    );
}

// E. bungeanus 0.65 2.07 3.20 0.46 1.46
// E. europaeus 0.61 1.78 2.90 0.46 1.34
// E. hamiltonianus 0.91 1.81 2.00 0.58 1.16
// E. phellomanus 0.59 2.10 3.60 0.37 1.32
// E. semiexsertus 0.44 1.76 4.00 0.31 1.22
// E. sieboldianus 0.61 1.99 3.30 0.48 1.55
// E. alatus 0.78 1.65 2.20 0.57 1.21
// E. sacrosanctus 0.60 1.74 2.90 0.46 1.33
// E. pauciflorus 1.11 2.04 1.80 0.76 1.39
// E. latifolius 1.12 1.67 1.50 0.89 1.33
// E. macropterus 1.67 1.99 1.20 0.89 1.06
// E. maximowiczianus 1.35 1.87 1.40 0.84 1.17
// E. sachalinensis 1.45 2.59 1.8 0.89 1.59
