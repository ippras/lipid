use lipid::{
    polars::bound::identifiers::{D, DC, DT, S, T, TC, TT, U, UC, UT},
    prelude::*,
};
use polars::prelude::*;
use std::sync::LazyLock;

const ALL: [&str; 10] = [S, D, DC, DT, T, TC, TT, U, UC, UT];

pub static SOURCE: LazyLock<DataFrame> = LazyLock::new(|| {
    (|| {
        df! {
            "FattyAcid" => [
                Some(Series::from_iter(C14U0).cast(&IDENTIFIER_DATA_TYPE)?),
                Some(Series::from_iter(C18U0).cast(&IDENTIFIER_DATA_TYPE)?),
                Some(Series::from_iter(C18U2DC9DC12).cast(&IDENTIFIER_DATA_TYPE)?),
                Some(Series::from_iter(ALL).cast(&IDENTIFIER_DATA_TYPE)?),
                Some(Series::from_iter([Some(S), None]).cast(&IDENTIFIER_DATA_TYPE)?),
                None,
            ],
        }?
        .with_row_index(PlSmallStr::from_static("Index"), None)
    })()
    .unwrap()
});

#[test]
fn new() {
    let series = SOURCE["FattyAcid"].as_materialized_series();
    let fatty_acid_chunked = IndexedIdentifierListChunked::new(&series).unwrap();
    assert_eq!(fatty_acid_chunked.len(), 6);
}

#[test]
fn get() {
    let series = SOURCE["FattyAcid"].as_materialized_series();
    let fatty_acid_chunked = IndexedIdentifierListChunked::new(series).unwrap();
    let bound_series = fatty_acid_chunked.get(0).unwrap();
    assert_eq!(bound_series.len(), 13);
}

#[test]
fn is_saturated() {
    let series = SOURCE["FattyAcid"].as_materialized_series();
    let fatty_acid_chunked = IndexedIdentifierListChunked::new(series).unwrap();
    let is_saturated = fatty_acid_chunked.is_saturated().unwrap();
    assert_eq!(
        is_saturated.into_iter().collect::<Vec<_>>(),
        [
            Some(true),
            Some(true),
            Some(false),
            Some(false),
            Some(false),
            None,
        ],
    );
}

#[test]
fn is_unsaturated() {
    let series = SOURCE["FattyAcid"].as_materialized_series();
    let fatty_acid_chunked = IndexedIdentifierListChunked::new(series).unwrap();
    let is_unsaturated = fatty_acid_chunked.is_unsaturated(None).unwrap();
    assert_eq!(
        is_unsaturated.into_iter().collect::<Vec<_>>(),
        [
            Some(false),
            Some(false),
            Some(true),
            Some(true),
            Some(false),
            None,
        ],
    );
}

#[test]
fn filter_unsaturated() -> PolarsResult<()> {
    let series = SOURCE["FattyAcid"].as_materialized_series();
    let fatty_acid_chunked = IndexedIdentifierListChunked::new(series).unwrap();
    let saturated = fatty_acid_chunked.filter(&fatty_acid_chunked.is_saturated()?)?;
    assert_eq!(
        saturated.into_iter().collect::<Vec<_>>(),
        [
            Some(Series::from_iter(C14U0)),
            Some(Series::from_iter(C18U0)),
        ]
    );
    Ok(())
}

#[test]
fn nullify_unsaturated() -> PolarsResult<()> {
    let series = SOURCE["FattyAcid"].as_materialized_series();
    let fatty_acid_chunked = IndexedIdentifierListChunked::new(series).unwrap();
    let saturated = fatty_acid_chunked.nullify(&fatty_acid_chunked.is_saturated()?)?;
    assert_eq!(
        saturated.into_iter().collect::<Vec<_>>(),
        [
            Some(Series::from_iter(C14U0)),
            Some(Series::from_iter(C18U0)),
            None,
            None,
            None,
            None,
        ]
    );
    Ok(())
}

#[test]
fn filter_saturated() -> PolarsResult<()> {
    let series = SOURCE["FattyAcid"].as_materialized_series();
    let fatty_acid_chunked = IndexedIdentifierListChunked::new(series).unwrap();
    let unsaturated = fatty_acid_chunked.filter(&fatty_acid_chunked.is_unsaturated(None)?)?;
    assert_eq!(
        unsaturated.into_iter().collect::<Vec<_>>(),
        [
            Some(Series::from_iter(C18U2DC9DC12)),
            Some(Series::from_iter(ALL)),
        ]
    );
    Ok(())
}

#[test]
fn nullify_saturated() -> PolarsResult<()> {
    let series = SOURCE["FattyAcid"].as_materialized_series();
    let fatty_acid_chunked = IndexedIdentifierListChunked::new(series).unwrap();
    let unsaturated = fatty_acid_chunked.nullify(&fatty_acid_chunked.is_unsaturated(None)?)?;
    assert_eq!(
        unsaturated.into_iter().collect::<Vec<_>>(),
        [
            None,
            None,
            Some(Series::from_iter(C18U2DC9DC12)),
            Some(Series::from_iter(ALL)),
            None,
            None,
        ]
    );
    Ok(())
}

#[test]
fn unsaturation() {
    let series = SOURCE["FattyAcid"].as_materialized_series();
    let fatty_acid_chunked = IndexedIdentifierListChunked::new(series).unwrap();
    let unsaturation = fatty_acid_chunked.unsaturation().unwrap();
    assert_eq!(
        unsaturation.into_iter().collect::<Vec<_>>(),
        [Some(0), Some(0), Some(2), Some(9), Some(0), None,]
    );
}
