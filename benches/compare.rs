use criterion::{Criterion, black_box, criterion_group, criterion_main};
use lipid::{polars::bound::identifiers::*, prelude::*};
use polars::prelude::*;
use std::sync::LazyLock;

const C14U0: [&str; 13] = [S, S, S, S, S, S, S, S, S, S, S, S, S];
const C18U0: [&str; 17] = [S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S];
const C18U2C9T12: [&str; 17] = [S, S, S, S, S, S, S, S, DC, S, S, DC, S, S, S, S, S];

pub const SOURCE: LazyLock<PolarsResult<DataFrame>> = LazyLock::new(|| {
    df! {
        "Index" => [0, 1, 2],
        "FattyAcid" => [
            Series::from_iter(C14U0).cast(&Bound::DATA_TYPE)?,
            Series::from_iter(C18U0).cast(&Bound::DATA_TYPE)?,
            Series::from_iter(C18U2C9T12).cast(&Bound::DATA_TYPE)?,
        ],
    }
});

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

fn bounds() -> PolarsResult<()> {
    SOURCE
        .clone()?
        .lazy()
        .select([col("FattyAcid").fatty_acid().bounds()])
        .collect()?;
    Ok(())
}

fn is_unsaturated() -> PolarsResult<()> {
    SOURCE
        .clone()?
        .lazy()
        .select([col("FattyAcid").fatty_acid().is_unsaturated()])
        .collect()?;
    Ok(())
}

fn is_unsaturated1() -> DataFrame {
    SOURCE
        .clone()
        .unwrap()
        .lazy()
        .select([col("FattyAcid").fatty_acid().is_unsaturated()])
        .collect()
        .unwrap()
}

fn criterion_benchmark(criterion: &mut Criterion) {
    criterion.bench_function("bounds", |bencher| bencher.iter(|| bounds()));
    criterion.bench_function("is_unsaturated", |bencher| {
        bencher.iter(|| is_unsaturated())
    });
    criterion.bench_function("is_unsaturated1", |bencher| {
        bencher.iter(|| is_unsaturated1())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
