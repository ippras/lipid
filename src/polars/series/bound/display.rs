use crate::prelude::*;
use std::fmt::{self, Formatter};

/// Display
pub enum Display {
    Common(Common),
    System(System),
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Display::Common(Common::Delta(delta)) => fmt::Display::fmt(delta, f),
            Display::System(system) => fmt::Display::fmt(system, f),
        }
    }
}

/// Common display
pub enum Common {
    Delta(Delta),
}

/// Delta display
pub struct Delta {
    pub carbons: u8,
    pub unsaturated: Vec<(usize, Option<Unsaturated>)>,
    pub options: Options,
}

impl fmt::Display for Delta {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("c")?;
        fmt::Display::fmt(&self.carbons, f)?;
        f.write_str(":")?;
        fmt::Display::fmt(&self.unsaturated.len(), f)?;
        if f.alternate() {
            let mut iter = self.unsaturated.iter();
            if let Some(&(index, unsaturated)) = iter.next() {
                f.write_str("Δ")?;
                match unsaturated {
                    None => f.write_str("?")?,
                    Some(unsaturated) => {
                        CommonUnsaturated::new(index, unsaturated, self.options).fmt(f)?;
                        for &(index, unsaturated) in iter {
                            f.write_str(",")?;
                            match unsaturated {
                                None => f.write_str("?")?,
                                Some(unsaturated) => {
                                    CommonUnsaturated::new(index, unsaturated, self.options)
                                        .fmt(f)?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

/// Display common unsaturated
struct CommonUnsaturated {
    index: usize,
    unsaturated: Unsaturated,
    options: Options,
}

impl CommonUnsaturated {
    fn new(index: usize, unsaturated: Unsaturated, options: Options) -> Self {
        Self {
            index,
            unsaturated,
            options,
        }
    }
}

impl fmt::Display for CommonUnsaturated {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.unsaturated.unsaturation {
            None => f.write_str("?")?,
            Some(Unaturation::Double) => {
                if self.options.bounds == Elision::Explicit {
                    f.write_str("=")?;
                }
            }
            Some(Unaturation::Triple) => f.write_str("≡")?,
        };
        fmt::Display::fmt(&self.index, f)?;
        match self.unsaturated.isomerism {
            None => f.write_str("?")?,
            Some(Isomerism::Cis) => {
                if self.options.isomerism == Elision::Explicit {
                    f.write_str("c")?;
                }
            }
            Some(Isomerism::Trans) => f.write_str("t")?,
        };
        Ok(())
    }
}

// fn common_unsaturated(
//     f: &mut Formatter,
//     index: &usize,
//     unsaturated: &Unsaturated,
//     options: &Options,
// ) -> fmt::Result {
//     match unsaturated.unsaturation {
//         None => f.write_str("?")?,
//         Some(Unaturation::Double) => {
//             if options.bounds == Elision::Explicit {
//                 f.write_str("=")?;
//             }
//         }
//         Some(Unaturation::Triple) => f.write_str("≡")?,
//     };
//     fmt::Display::fmt(&index, f)?;
//     match unsaturated.isomerism {
//         None => f.write_str("?")?,
//         Some(Isomerism::Cis) => {
//             if options.isomerism == Elision::Explicit {
//                 f.write_str("c")?;
//             }
//         }
//         Some(Isomerism::Trans) => f.write_str("t")?,
//     };
//     Ok(())
// }

/// Display system
pub struct System {
    pub carbons: u8,
    pub unsaturated: Vec<(usize, Option<Unsaturated>)>,
}

// c18u3dc9dc12dc15
impl fmt::Display for System {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("c")?;
        write!(f, "{:02}", self.carbons)?;
        f.write_str("u")?;
        write!(f, "{:02}", self.unsaturated.len())?;
        for &(index, unsaturated) in &self.unsaturated {
            write!(f, "{}", SystemUnsaturated::new(index, unsaturated))?;
        }
        Ok(())
    }
}

/// Display system unsaturated
struct SystemUnsaturated {
    index: usize,
    unsaturated: Option<Unsaturated>,
}

impl SystemUnsaturated {
    fn new(index: usize, unsaturated: Option<Unsaturated>) -> Self {
        Self { index, unsaturated }
    }
}

impl fmt::Display for SystemUnsaturated {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self
            .unsaturated
            .and_then(|unsaturated| unsaturated.unsaturation)
        {
            None => f.write_str("u")?,
            Some(Unaturation::Double) => f.write_str("d")?,
            Some(Unaturation::Triple) => f.write_str("t")?,
        };
        match self
            .unsaturated
            .and_then(|unsaturated| unsaturated.isomerism)
        {
            None => f.write_str("i")?,
            Some(Isomerism::Cis) => f.write_str("c")?,
            Some(Isomerism::Trans) => f.write_str("t")?,
        };
        write!(f, "{:02}", self.index)
    }
}

/// Elision
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Elision {
    Explicit,
    #[default]
    Implicit,
}

/// Display options
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Options {
    pub kind: Kind,
    pub bounds: Elision,
    pub isomerism: Elision,
}

/// Display kind
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Kind {
    #[default]
    Delta,
    System,
}
