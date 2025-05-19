use criterion::{Criterion, black_box, criterion_group, criterion_main};
use lipid::{polars::bound::identifiers::*, prelude::*};
use polars::prelude::*;
use std::{
    num::{NonZeroI8, NonZeroU8},
    sync::LazyLock,
};

// const C100U0: [&str; 100] = [
//     S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S,
//     S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S,
//     S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S,
//     S, S, S, S,
// ];
const C100U0: [&str; 33] = [
    S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, U, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S,
    S,
];

pub static SOURCE: LazyLock<DataFrame> = LazyLock::new(|| {
    df! {
        "FattyAcid" => [
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
            FattyAcidChunked::try_from(C100U0).unwrap().into_struct(PlSmallStr::EMPTY).unwrap().into_series(),
        ],
    }
    .unwrap()
});

fn is_delta_unsaturated() -> () {
    let _ = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .is_delta_unsaturated(NonZeroU8::new(17).unwrap())])
        .collect()
        .unwrap();
}

fn is_omega_unsaturated() -> () {
    let _ = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .is_omega_unsaturated(NonZeroU8::new(17).unwrap())])
        .collect()
        .unwrap();
}

fn criterion_benchmark(criterion: &mut Criterion) {
    criterion.bench_function("is_delta_unsaturated", |bencher| {
        bencher.iter(|| is_delta_unsaturated())
    });
    criterion.bench_function("is_omega_unsaturated", |bencher| {
        bencher.iter(|| is_omega_unsaturated())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// fn expression() -> DataFrame {
//     SOURCE
//         .clone()
//         .unwrap()
//         .lazy()
//         .select([col("FattyAcid").fatty_acid().is_unsaturated()])
//         .collect()
//         .unwrap()
// }

// fn function() -> DataFrame {
//     SOURCE
//         .clone()
//         .unwrap()
//         .lazy()
//         .select([col("FattyAcid").fatty_acid().is_unsaturated_function()])
//         .collect()
//         .unwrap()
// }

// fn criterion_benchmark(criterion: &mut Criterion) {
//     criterion.bench_function("expression", |bencher| bencher.iter(|| expression()));
//     criterion.bench_function("function", |bencher| bencher.iter(|| function()));
// }

// fn bounds() -> PolarsResult<()> {
//     SOURCE
//         .clone()?
//         .lazy()
//         .select([col("FattyAcid").fatty_acid().bounds()])
//         .collect()?;
//     Ok(())
// }

// fn is_unsaturated() -> PolarsResult<()> {
//     SOURCE
//         .clone()?
//         .lazy()
//         .select([col("FattyAcid").fatty_acid().is_unsaturated()])
//         .collect()?;
//     Ok(())
// }

// fn is_unsaturated1() -> DataFrame {
//     SOURCE
//         .clone()
//         .unwrap()
//         .lazy()
//         .select([col("FattyAcid").fatty_acid().is_unsaturated()])
//         .collect()
//         .unwrap()
// }
