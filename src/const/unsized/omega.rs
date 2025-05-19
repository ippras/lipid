use crate::prelude::*;
use std::num::NonZeroI8;

/// ω-3
pub static OMEGA3: [(Option<Option<NonZeroI8>>, Option<&str>); 4] = [
    (None, None),
    (Some(NonZeroI8::new(-3)), Some(U)),
    (Some(NonZeroI8::new(-2)), Some(S)),
    (Some(NonZeroI8::new(-1)), Some(S)),
];

/// ω-4
pub static OMEGA4: [(Option<Option<NonZeroI8>>, Option<&str>); 5] = [
    (None, None),
    (Some(NonZeroI8::new(-4)), Some(U)),
    (Some(NonZeroI8::new(-3)), Some(S)),
    (Some(NonZeroI8::new(-2)), Some(S)),
    (Some(NonZeroI8::new(-1)), Some(S)),
];

/// ω-5
pub static OMEGA5: [(Option<Option<NonZeroI8>>, Option<&str>); 6] = [
    (None, None),
    (Some(NonZeroI8::new(-5)), Some(U)),
    (Some(NonZeroI8::new(-4)), Some(S)),
    (Some(NonZeroI8::new(-3)), Some(S)),
    (Some(NonZeroI8::new(-2)), Some(S)),
    (Some(NonZeroI8::new(-1)), Some(S)),
];

/// ω-6
pub static OMEGA6: [(Option<Option<NonZeroI8>>, Option<&str>); 7] = [
    (None, None),
    (Some(NonZeroI8::new(-6)), Some(U)),
    (Some(NonZeroI8::new(-5)), Some(S)),
    (Some(NonZeroI8::new(-4)), Some(S)),
    (Some(NonZeroI8::new(-3)), Some(S)),
    (Some(NonZeroI8::new(-2)), Some(S)),
    (Some(NonZeroI8::new(-1)), Some(S)),
];

/// ω-7
pub static OMEGA7: [(Option<Option<NonZeroI8>>, Option<&str>); 8] = [
    (None, None),
    (Some(NonZeroI8::new(-7)), Some(U)),
    (Some(NonZeroI8::new(-6)), Some(S)),
    (Some(NonZeroI8::new(-5)), Some(S)),
    (Some(NonZeroI8::new(-4)), Some(S)),
    (Some(NonZeroI8::new(-3)), Some(S)),
    (Some(NonZeroI8::new(-2)), Some(S)),
    (Some(NonZeroI8::new(-1)), Some(S)),
];

/// ω-8
pub static OMEGA8: [(Option<Option<NonZeroI8>>, Option<&str>); 9] = [
    (None, None),
    (Some(NonZeroI8::new(-8)), Some(U)),
    (Some(NonZeroI8::new(-7)), Some(S)),
    (Some(NonZeroI8::new(-6)), Some(S)),
    (Some(NonZeroI8::new(-5)), Some(S)),
    (Some(NonZeroI8::new(-4)), Some(S)),
    (Some(NonZeroI8::new(-3)), Some(S)),
    (Some(NonZeroI8::new(-2)), Some(S)),
    (Some(NonZeroI8::new(-1)), Some(S)),
];

/// ω-9
pub static OMEGA9: [(Option<Option<NonZeroI8>>, Option<&str>); 10] = [
    (None, None),
    (Some(NonZeroI8::new(-9)), Some(U)),
    (Some(NonZeroI8::new(-8)), Some(S)),
    (Some(NonZeroI8::new(-7)), Some(S)),
    (Some(NonZeroI8::new(-6)), Some(S)),
    (Some(NonZeroI8::new(-5)), Some(S)),
    (Some(NonZeroI8::new(-4)), Some(S)),
    (Some(NonZeroI8::new(-3)), Some(S)),
    (Some(NonZeroI8::new(-2)), Some(S)),
    (Some(NonZeroI8::new(-1)), Some(S)),
];

/// ω-10
pub static OMEGA10: [(Option<Option<NonZeroI8>>, Option<&str>); 11] = [
    (None, None),
    (Some(NonZeroI8::new(-10)), Some(U)),
    (Some(NonZeroI8::new(-9)), Some(S)),
    (Some(NonZeroI8::new(-8)), Some(S)),
    (Some(NonZeroI8::new(-7)), Some(S)),
    (Some(NonZeroI8::new(-6)), Some(S)),
    (Some(NonZeroI8::new(-5)), Some(S)),
    (Some(NonZeroI8::new(-4)), Some(S)),
    (Some(NonZeroI8::new(-3)), Some(S)),
    (Some(NonZeroI8::new(-2)), Some(S)),
    (Some(NonZeroI8::new(-1)), Some(S)),
];

/// ω-11
pub static OMEGA11: [(Option<Option<NonZeroI8>>, Option<&str>); 12] = [
    (None, None),
    (Some(NonZeroI8::new(-11)), Some(U)),
    (Some(NonZeroI8::new(-10)), Some(S)),
    (Some(NonZeroI8::new(-9)), Some(S)),
    (Some(NonZeroI8::new(-8)), Some(S)),
    (Some(NonZeroI8::new(-7)), Some(S)),
    (Some(NonZeroI8::new(-6)), Some(S)),
    (Some(NonZeroI8::new(-5)), Some(S)),
    (Some(NonZeroI8::new(-4)), Some(S)),
    (Some(NonZeroI8::new(-3)), Some(S)),
    (Some(NonZeroI8::new(-2)), Some(S)),
    (Some(NonZeroI8::new(-1)), Some(S)),
];

/// ω-12
pub static OMEGA12: [(Option<Option<NonZeroI8>>, Option<&str>); 13] = [
    (None, None),
    (Some(NonZeroI8::new(-12)), Some(U)),
    (Some(NonZeroI8::new(-11)), Some(S)),
    (Some(NonZeroI8::new(-10)), Some(S)),
    (Some(NonZeroI8::new(-9)), Some(S)),
    (Some(NonZeroI8::new(-8)), Some(S)),
    (Some(NonZeroI8::new(-7)), Some(S)),
    (Some(NonZeroI8::new(-6)), Some(S)),
    (Some(NonZeroI8::new(-5)), Some(S)),
    (Some(NonZeroI8::new(-4)), Some(S)),
    (Some(NonZeroI8::new(-3)), Some(S)),
    (Some(NonZeroI8::new(-2)), Some(S)),
    (Some(NonZeroI8::new(-1)), Some(S)),
];

// fn omega(index: i8) -> PolarsResult<Series> {
//     let s = index.abs() as usize - 1;
//     Ok(df! {
//         INDEX => Series::from_iter(once(None).chain((index..0).map(Some))),
//         IDENTIFIER => Series::from_iter(once(None).chain(once(Some(U)).chain(repeat_n(Some(S), s)))).cast(&BOUND_DATA_TYPE)?,
//     }?
//     .into_struct(PlSmallStr::EMPTY).into_series())
// }
