use criterion::{Criterion, black_box, criterion_group, criterion_main};
use lipid::{polars::bound::identifiers::*, prelude::*};
use polars::prelude::*;
use std::sync::LazyLock;

const C100U0: [&str; 100] = [
    S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S,
    S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S,
    S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S,
    S, S, S, S,
];

pub static SOURCE: LazyLock<DataFrame> = LazyLock::new(|| {
    df! {
        "FattyAcid" => [
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
            Series::from_iter(C100U0).cast(&BOUND_DATA_TYPE).unwrap(),
        ],
    }
    .unwrap()
});

fn unsaturated1() -> () {
    let _ = SOURCE
        .fatty_acid()
        .unwrap()
        .unsaturated1(true)
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
}

fn unsaturated2() -> () {
    let _ = SOURCE
        .fatty_acid()
        .unwrap()
        .unsaturated2(true)
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
}

fn criterion_benchmark(criterion: &mut Criterion) {
    criterion.bench_function("unsaturated1", |bencher| bencher.iter(|| unsaturated1()));
    criterion.bench_function("unsaturated2", |bencher| bencher.iter(|| unsaturated2()));
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
