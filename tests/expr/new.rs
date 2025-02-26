use lipid::{polars::prelude::BOUND_DATA_TYPE, prelude::*};
use polars::prelude::*;
use polars_arrow::array::{BinaryViewArrayGeneric, Utf8ViewArray};
use std::{borrow::Cow, iter::empty, sync::LazyLock};

const C14U0: [Option<i8>; 14] = [
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
];

const C18U0: [Option<i8>; 18] = [
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
];

const C18U2C9T12: [Option<i8>; 18] = [
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(1),
    Some(0),
    Some(0),
    Some(1),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
    Some(0),
];

#[test]
pub fn test() {
    let enum_letters = ["a", "b", "c", "d"];
    let s = enum_letters.iter().map(Some).collect::<Vec<_>>();
    let categories = Utf8ViewArray::from_slice(s);
    let rev_mapping = RevMapping::build_local(categories);

    let letters = ["a", "b", "c", "d", "a", "b", "c", "d"];
    let series = Series::new("letters".into(), letters);
    let actual = series
        .cast(&DataType::Enum(
            Some(rev_mapping.into()),
            CategoricalOrdering::Physical,
        ))
        .unwrap();
    let actual = actual.to_physical_repr();
    let expected = Series::new("letters".into(), &[0, 1, 2, 3, 0, 1, 2, 3]);
    assert_eq!(actual, Cow::Owned(expected), "Comparing enum series");
}

#[test]
fn find() -> PolarsResult<()> {
    // let array = Utf8Array::<i32>::from([Some("hi"), None, Some("there")]);
    // let t = BinaryViewArrayGeneric;

    // let enum_letters = ["a", "b", "c", "d"];
    // let letters = &["s", "d", "t"];
    // let categories = Utf8ViewArray::from_slice_values(letters);
    // let bound_data_type = create_enum_dtype(categories);
    let data_frame = df! {
        // "FattyAcid" => &[
        //     Series::from_iter(C14U0),
        //     Series::from_iter(C18U0),
        //     Series::from_iter(C18U2C9T12),
        // ],
        // "Value" => [
        //     Some(1f64),
        //     Some(2f64),
        //     Some(3f64),
        // ],
        "V" => [
            Series::from_iter(["s", "s", "f"]).cast(&BOUND_DATA_TYPE)?,
        ],
    }?;
    println!("data_frame: {}", data_frame);
    // println!("data_frame: {}", data_frame.unnest(["FattyAcid"])?);
    // let mut lazy_frame = data_frame.lazy();
    // // lazy_frame = lazy_frame.with_columns([col("FattyAcid").fa().ecn().alias("ECN")]);
    // println!("lazy_frame: {}", lazy_frame.clone().collect()?);
    Ok(())
}
