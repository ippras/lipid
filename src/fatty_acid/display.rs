use crate::fatty_acid::FattyAcid;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    fmt::{self, Formatter},
};

pub const ID: Options = Options {
    separators: Separators {
        c: "c",
        u: "u",
        i: ["", ""],
    },
    notation: Notation::Prefix,
    elision: Elision::Explicit,
};

pub const COMMON: Options = Options {
    separators: Separators {
        c: "",
        u: ":",
        i: ["Δ", ","],
    },
    notation: Notation::Suffix,
    elision: Elision::Implicit,
};

/// Display with options
pub trait DisplayWithOptions {
    fn display(self, options: Options) -> Display<Self>
    where
        Self: Sized;
}

impl DisplayWithOptions for FattyAcid {
    fn display(self, options: Options) -> Display<FattyAcid> {
        Display::new(self, options)
    }
}

impl<'a> DisplayWithOptions for &'a mut FattyAcid {
    fn display(self, options: Options) -> Display<&'a mut FattyAcid> {
        Display::new(self, options)
    }
}

impl<'a> DisplayWithOptions for &'a FattyAcid {
    fn display(self, options: Options) -> Display<&'a FattyAcid> {
        Display::new(self, options)
    }
}

/// Fatty acid display
#[derive(Clone, Debug)]
pub struct Display<T> {
    fatty_acid: T,
    options: Options,
}

impl<T> Display<T> {
    pub fn new(fatty_acid: T, options: Options) -> Self {
        Self {
            fatty_acid,
            options,
        }
    }
}

// impl<T: Borrow<FattyAcid>> fmt::Display for Display<T> {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         let fatty_acid = self.fatty_acid.borrow();
//         f.write_str(self.options.separators.c)?;
//         fmt::Display::fmt(&fatty_acid.carbons, f)?;
//         let point = fatty_acid
//             .unsaturated
//             .partition_point(|unsaturated| unsaturated.unsaturation == Some(Unsaturation::One));
//         let doubles = &fatty_acid.unsaturated.as_slice()[..point];
//         let triples = &fatty_acid.unsaturated.as_slice()[point..];
//         println!("fatty_acid.unsaturated {:?}", fatty_acid.unsaturated,);
//         // unsaturated
//         f.write_str(self.options.separators.u)?;
//         fmt::Display::fmt(&doubles.len(), f)?;
//         if !triples.is_empty() {
//             f.write_str(self.options.separators.u)?;
//             fmt::Display::fmt(&triples.len(), f)?;
//         }
//         if f.alternate() {
//             let mut iter = doubles.into_iter().chain(triples);
//             // if let Some(unsaturated) = iter.next() {
//             //     f.write_str(self.options.separators.i[0])?;
//             //     fmt::Display::fmt(
//             //         &index::Display::new(
//             //             unsaturated.index + 1,
//             //             isomerism::Display::new(unsaturated.isomerism, self.options.elision),
//             //             self.options.notation,
//             //         ),
//             //         f,
//             //     )?;
//             //     for unsaturated in iter {
//             //         f.write_str(self.options.separators.i[1])?;
//             //         fmt::Display::fmt(
//             //             &index::Display::new(
//             //                 unsaturated.index + 1,
//             //                 isomerism::Display::new(unsaturated.isomerism, self.options.elision),
//             //                 self.options.notation,
//             //             ),
//             //             f,
//             //         )?;
//             //     }
//             // }
//         }
//         Ok(())
//     }
// }
impl<T: Borrow<FattyAcid>> fmt::Display for Display<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let fatty_acid = self.fatty_acid.borrow();
        f.write_str(self.options.separators.c)?;
        fmt::Display::fmt(&fatty_acid.carbons, f)?;
        f.write_str(self.options.separators.u)?;
        fmt::Display::fmt(&fatty_acid.unsaturated.len(), f)?;
        if f.alternate() {
            let mut iter = fatty_acid.unsaturated.iter();
            if let Some(unsaturated) = iter.next() {
                f.write_str(self.options.separators.i[0])?;
                fmt::Display::fmt(
                    &unsaturated::Display::new(
                        unsaturated,
                        self.options.notation,
                        self.options.elision,
                    ),
                    f,
                )?;
                for unsaturated in iter {
                    f.write_str(self.options.separators.i[1])?;
                    fmt::Display::fmt(
                        &unsaturated::Display::new(
                            unsaturated,
                            self.options.notation,
                            self.options.elision,
                        ),
                        f,
                    )?;
                }
            }
        }
        Ok(())
    }
}

/// Display options
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Options {
    pub separators: Separators,
    pub notation: Notation,
    pub elision: Elision,
}

/// Separators
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Separators {
    pub c: &'static str,
    pub u: &'static str,
    pub i: [&'static str; 2],
}

/// Isomerism notation
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Notation {
    Prefix,
    Suffix,
}

/// Isomerism elision
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Elision {
    Explicit,
    #[default]
    Implicit,
}

mod unsaturated {
    use super::{Elision, Notation, affix};
    use crate::fatty_acid::Unsaturated;
    use std::fmt::{self, Formatter};

    /// Unsaturated display
    pub(super) struct Display<'a> {
        unsaturated: &'a Unsaturated,
        notation: Notation,
        elision: Elision,
    }

    impl<'a> Display<'a> {
        pub(super) fn new(
            unsaturated: &'a Unsaturated,
            notation: Notation,
            elision: Elision,
        ) -> Self {
            Self {
                unsaturated,
                notation,
                elision,
            }
        }
    }

    impl fmt::Display for Display<'_> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let index = |f| {
                if let Some(index) = self.unsaturated.index {
                    fmt::Display::fmt(&index, f)
                } else {
                    f.write_str("*")
                }
            };
            let isomerism = self.unsaturated.isomerism.unwrap_or_default();
            let unsaturation = self.unsaturated.unsaturation.unwrap_or_default();
            match self.notation {
                Notation::Prefix => {
                    fmt::Display::fmt(
                        &affix::Display::new(isomerism, unsaturation, self.elision),
                        f,
                    )?;
                    index(f)?;
                }
                Notation::Suffix => {
                    index(f)?;
                    fmt::Display::fmt(
                        &affix::Display::new(isomerism, unsaturation, self.elision),
                        f,
                    )?;
                }
            }
            Ok(())
        }
    }
}

mod affix {
    use super::Elision;
    use crate::fatty_acid::{Isomerism, Unsaturation};
    use std::fmt::{self, Formatter, Write};

    /// Display affix
    pub(super) struct Display {
        pub(super) isomerism: Isomerism,
        pub(super) unsaturation: Unsaturation,
        pub(super) elision: Elision,
    }

    impl Display {
        pub(super) fn new(
            isomerism: Isomerism,
            unsaturation: Unsaturation,
            elision: Elision,
        ) -> Self {
            Self {
                isomerism,
                unsaturation,
                elision,
            }
        }
    }

    impl fmt::Display for Display {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self.isomerism {
                Isomerism::Cis => {
                    if self.elision == Elision::Explicit || self.unsaturation == Unsaturation::Two {
                        match self.unsaturation {
                            Unsaturation::One => f.write_char('c')?,
                            Unsaturation::Two => f.write_str("ct")?,
                        }
                    }
                }
                Isomerism::Trans => match self.unsaturation {
                    Unsaturation::One => f.write_char('t')?,
                    Unsaturation::Two => f.write_str("tt")?,
                },
            }
            Ok(())
        }
    }
}

mod isomerism {
    use super::Elision;
    use crate::fatty_acid::Isomerism;
    use std::fmt::{self, Formatter, Write};

    /// Display isomerism
    pub(super) struct Display {
        pub(super) isomerism: Isomerism,
        pub(super) elision: Elision,
    }

    impl Display {
        pub(super) fn new(isomerism: Isomerism, elision: Elision) -> Self {
            Self { isomerism, elision }
        }
    }

    impl fmt::Display for Display {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self.isomerism {
                Isomerism::Cis => {
                    if self.elision == Elision::Explicit {
                        f.write_char('c')?;
                    }
                }
                Isomerism::Trans => {
                    f.write_char('t')?;
                }
            }
            Ok(())
        }
    }
}
