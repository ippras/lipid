#![feature(formatting_options)]

use anyhow::Result;
use lipid::prelude::*;
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

// FA C*:*
// SFA C*:0
// MUFA C*:1
// PUFA C*:2+

// "CCCCCCCCCCCCCCCC(=O)O"
// "CCCCCCCCCCCCCCCCCC(=O)O"
// Oleic "CCCCCCCCC=CCCCC(=O)O"
// Omega -> CCC=CCC=CCC=CCCCCCCCC(=O)O <- Delta

// W3 C*:1+[*,D-3]
// W6 C*:1+[*,D-6]
// W9 C*:1+[*,D-9]

// C18:3[D9,D12,D15]

// FA    *:*  [{None, None}]
// SFA   *:0= [{None, S}]
// MUFA 1+:1= [{0, U}, {None, S}]
// PUFA 2+:2+ [{0, U}, {0, U}, {None, S}]
//      15:1= [{0, S}, {0, S}, {0, S}, {0, S}, {0, S}, {0, S}, {0, S}, {0, S}, {0, S}, {0, S}, {0, S}, {0, S}, {0, S}, {0, S}, {0, U}]
// ω3   3+:1+ [{-3, U}, {-2, S}, {-1, S}, {None, None}]
// ω6   6+:1+ [{-6, U}, {-5, S}, {-4, S}, {-3, S}, {-2, S}, {-1, S}, {None, None}]
// ω9   9+:1+ [{-9, U}, {-8, S}, {-7, S}, {-6, S}, {-5, S}, {-4, S}, {-3, S}, {-2, S}, {-1, S}, {None, None}]

// Index
// None: any unknown indices
// Some(0): one unknown index
// Some(x): one known index

// PUFA
// Series::from_iter([None, None, None, None, None])
// Series::from_iter([S   , U   , S   , U   , S   ])
// Series::from_iter([None, 1   , None, 1   , None])

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

pub const INDEX: &str = FattyAcidChunked::INDEX;
pub const IDENTIFIER: &str = FattyAcidChunked::IDENTIFIER;
pub const OMEGA_3: LazyLock<BoundChunked> =
    LazyLock::new(|| BoundChunked::try_from(&[DC, S, S]).unwrap());

#[test]
fn test1() -> Result<()> {
    // unsafe { std::env::set_var("POLARS_FMT_MAX_ROWS", "256") };
    unsafe { std::env::set_var("POLARS_FMT_TABLE_CELL_LIST_LEN", "256") };
    unsafe { std::env::set_var("POLARS_FMT_STR_LEN", "256") };

    let data_frame = df! {
        "FattyAcid" => [
            FattyAcidChunked::try_from(&C4)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C5)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C6)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C7)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C8)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C9)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C10)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C11)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C12)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C13)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C14)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C15)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C16)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C17)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C18)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C19)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C20)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C21)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C22)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C23)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C24)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C25)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C26)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C27)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C28)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C29)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C30)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C31)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C32)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C33)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C34)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C35)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C36)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C16DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C16DT9)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C18DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C18DT9)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C18DC9DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C18DC9DC12DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C18DC6DC9DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C18DC8DT10DC12)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C18DC9DT11DT13)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C18DT9DT11DT13)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C18DT9DT11DC13)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C18DC6DC9DC12DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C20DC9)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C20DC11)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C20DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C20DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C20DC8DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C20DC5DC8DC11)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C20DC5DC8DC11DC14)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C20DC8DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C20DC5DC8DC11DC14DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C22DC13)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C22DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C22DC5DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C22DC7DC10DC13DC16)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C22DC7DC10DC13DC16DC19)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C22DC4DC7DC10DC13DC16DC19)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C24DC15)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C24DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C24DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C24DC9DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C24DC6DC9DC12DC15DC18)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C24DC6DC9DC12DC15DC18DC21)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C26DC17)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
            FattyAcidChunked::try_from(&C30DC21)?.into_struct(PlSmallStr::EMPTY)?.into_series(),
        ]
    }?;
    // println!("data_frame: {data_frame}");
    for fatty_acid in data_frame.fatty_acid() {
        println!(
            "fatty_acid: {:#}",
            fatty_acid.display(Default::default()),
            // ,
            // ,
            // fatty_acid.is_delta(NonZero::new(9).unwrap()),
            // fatty_acid.is_omega_unsaturated(NonZero::new(3).unwrap()),
        );
    }
    // let indices = Series::from_iter([
    //     Some(1),
    //     Some(2),
    //     Some(3),
    //     None,
    //     Some(-3),
    //     Some(-2),
    //     Some(-1),
    // ]);
    // println!("indices: {indices}");
    // let indices = indices.sort_with(SortOptions::default().with_nulls_last(true))?;
    // println!("sort indices: {indices}");

    // let data_frame = df! {
    //     FattyAcidChunked::INDEX => Series::from_iter(repeat_n(0i8, 18)),
    //     FattyAcidChunked::IDENTIFIER => Series::from_iter(repeat_n(S, 18 - 2).chain(repeat_n(U, 2))).cast(&BOUND_DATA_TYPE)?,
    // }?;
    pub static TEST: [(Option<Option<NonZeroI8>>, Option<&str>); 9] = [
        (Some(None), Some(S)),
        (Some(NonZeroI8::new(9)), Some(S)),
        (None, Some(S)),
        (Some(None), Some(S)),
        (Some(NonZeroI8::new(15)), None),
        (Some(None), Some(U)),
        (Some(None), None),
        (Some(NonZeroI8::new(12)), Some(U)),
        (Some(None), Some(S)),
    ];
    let fatty_acid = FattyAcidChunked::try_from(&TEST)?;
    println!("fatty_acid: {fatty_acid:#}");
    println!("sized: {}", fatty_acid.sized_count());
    println!("unsized: {}", fatty_acid.unsized_count());
    println!("explicit: {}", fatty_acid.explicit_count());
    println!("implicit: {}", fatty_acid.implicit_count());
    println!("saturated: {}", fatty_acid.saturated_count());
    println!("unsaturated: {}", fatty_acid.unsaturated_count());

    println!("fatty_acid: {fatty_acid:#} => {:#}", fatty_acid.sort());
    let fatty_acid = fatty_acid.sort();
    let (r#unsized, sized) = fatty_acid.sized();
    println!("r#unsized: {unsized:?}");
    println!("sized: {:#}", sized);
    let (implicit, explicit) = sized.explicit();
    println!("implicit: {}", implicit);
    println!("explicit: {:#}", explicit);

    // df! {
    //     FattyAcidChunked::INDEX => Series::from_iter(repeat_n(0i8, b)),
    //     FattyAcidChunked::IDENTIFIER => Series::from_iter(repeat_n(U, u).chain(repeat_n(S, s))).cast(&BOUND_DATA_TYPE)?,
    // }?
    // .into_struct(PlSmallStr::EMPTY).into_series();
    // let data_frame = df! {
    //     INDEX => Series::from_iter([1, 2, 3, -3, -2, -1]),
    //     INDEX => Series::from_iter([1, 2, 3, -3, -2, -1]),
    // };

    //     *
    // 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19;

    // println!("omega_3: {}", OMEGA_3.clone().into_series());
    // let tail = FattyAcidChunked::try_from(&C18DC9DC12DC15)?
    //     .bound()
    //     .physical()
    //     .tail(Some(3));
    // println!("tail: {}", tail.clone().into_series());
    // println!(
    //     "tail == omega_3: {}",
    //     tail.equal_missing(OMEGA_3.physical()).all()
    // );
    Ok(())
}
