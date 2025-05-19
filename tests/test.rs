#![feature(formatting_options)]

use anyhow::Result;
use lipid::{polars::FATTY_ACID, prelude::*};
use polars::prelude::*;
use polars_arrow::array::MutableBinaryViewArray;
use std::{
    collections::BTreeMap,
    default,
    fmt::Sign,
    iter::{once, repeat, repeat_n},
    num::{NonZero, NonZeroI8},
    sync::LazyLock,
};

#[test]
fn test() -> Result<()> {
    let mut categories = MutableBinaryViewArray::<str>::new();
    let data_type = DataType::Enum(
        Some(Arc::new(RevMapping::build_local(categories.into()))),
        Default::default(),
    );
    let data_frame = df! {
        "FattyAcid" => Series::from_iter([
            r#"CC/C=C\C/C=C\C/C=C\CCCCCCCC(=O)O"#,
        ]),
    }?;
    println!("data_frame: {data_frame}");
    for fatty_acid in data_frame["FattyAcid"].as_materialized_series().str()? {
        let fatty_acid = fatty_acid.unwrap();
        let fatty_acid = fatty_acid
            .trim_end_matches("(=O)O")
            .replace(&['/', '\\'], "");
        println!("fatty_acid: {fatty_acid:?}");
        let c = fatty_acid.matches('C');
        println!("c: {:?}", c.count());
        let u = fatty_acid.match_indices(&['=', '#']);
        println!("u: {:?}", u.clone().count());
        // "CCC=CCC=CCC=CCCCCCCCC"
        for (index, (position, _)) in u.enumerate() {
            println!("u: {}", position - index);
        }

        // println!("{index:?}: {:?}", unsaturated.iter().collect::<Vec<_>>());
    }
    Ok(())
}

pub const INDEX: &str = FattyAcidChunked::INDICES;
pub const IDENTIFIER: &str = FattyAcidChunked::BOUNDS;
pub const OMEGA_3: LazyLock<BoundChunked> =
    LazyLock::new(|| BoundChunked::try_from([DC, S, S]).unwrap());

#[test]
fn test1() -> Result<()> {
    // unsafe { std::env::set_var("POLARS_FMT_MAX_ROWS", "256") };
    unsafe { std::env::set_var("POLARS_FMT_TABLE_CELL_LIST_LEN", "256") };
    unsafe { std::env::set_var("POLARS_FMT_STR_LEN", "256") };

    let data_frame = df! {
        FATTY_ACID => [
            // FattyAcidChunked::try_from(C4)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C5)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C6)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C7)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C8)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C9)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C10)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C11)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C12)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C13)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C14)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C15)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C16)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C17)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C18)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C19)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C20)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C21)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C22)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C23)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C24)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C25)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C26)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C27)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C28)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C29)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C30)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C31)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C32)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C33)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C34)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C35)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C36)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C16DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C16DT9)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C18DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C18DT9)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C18DC9DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(C18DC9DC12DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C18DC6DC9DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C18DC8DT10DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C18DC9DT11DT13)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C18DT9DT11DT13)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C18DT9DT11DC13)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C18DC6DC9DC12DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C20DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C20DC11)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C20DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C20DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C20DC8DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C20DC5DC8DC11)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C20DC5DC8DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C20DC8DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C20DC5DC8DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C22DC13)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C22DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C22DC5DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C22DC7DC10DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C22DC7DC10DC13DC16DC19)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C22DC4DC7DC10DC13DC16DC19)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C24DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C24DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C24DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C24DC9DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C24DC6DC9DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C24DC6DC9DC12DC15DC18DC21)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C26DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            // FattyAcidChunked::try_from(C30DC21)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
        ]
    }?;
    // println!("data_frame: {data_frame}");
    // println!(
    //     "schema: {:?}",
    //     data_frame
    //         .explode([FATTY_ACID])?
    //         .unnest([FATTY_ACID])?
    //         .schema()
    // );
    for fatty_acid in data_frame.fatty_acid_list() {
        println!(
            "fatty_acid: {fatty_acid} {:#} {:?}",
            fatty_acid.display(Default::default()),
            fatty_acid.is_cis(),
            // fatty_acid.is_delta_unsaturated(NonZero::new(9).unwrap())?,
            // fatty_acid.is_omega_unsaturated_old(NonZero::new(3).unwrap()),
            // fatty_acid.is_omega_unsaturated(NonZero::new(3).unwrap()),
        );
    }
    pub static TEST: [(Option<Option<NonZeroI8>>, Option<&str>); 9] = [
        (Some(None), Some(S)),
        (Some(NonZeroI8::new(9)), Some(S)),
        (None, Some(S)),
        (Some(None), Some(S)),
        (Some(NonZeroI8::new(-3)), None),
        (Some(None), Some(U)),
        (Some(None), None),
        (Some(NonZeroI8::new(12)), Some(U)),
        (Some(None), Some(S)),
    ];
    let fatty_acid = FattyAcidChunked::try_from(TEST)?;
    println!("fatty_acid: {fatty_acid:#}");
    println!("sized: {}", fatty_acid.sized_count());
    println!("unsized: {}", fatty_acid.unsized_count());
    println!("explicit: {}", fatty_acid.explicit_count());
    println!("implicit: {}", fatty_acid.implicit_count());
    println!("saturated: {}", fatty_acid.saturated_count());
    println!("unsaturated: {}", fatty_acid.unsaturated_count());

    assert_eq!(fatty_acid.is_sized(), false);
    assert_eq!(fatty_acid.is_unsized(), true);

    let sorted = fatty_acid.sort();
    println!("sorted: {fatty_acid:#} => {sorted:#}");
    let (r#unsized, sized) = sorted.unsized_sized();
    assert_eq!(sized.is_sized(), true);
    assert_eq!(sized.is_unsized(), false);
    assert_eq!(sized.is_explicit(), Some(false));
    assert_eq!(sized.is_implicit(), Some(true));

    println!("r#unsized: {unsized:?}");
    println!("sized: {:#}", sized);
    let (implicit, explicit) = sized.implicit_explicit();
    println!("implicit: {}", implicit);
    println!("explicit: {:#}", explicit);
    Ok(())
}
