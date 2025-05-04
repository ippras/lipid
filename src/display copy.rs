pub use self::{
    common::{Common, Delta},
    system::System,
};

use crate::prelude::*;
use std::fmt::{Display, Formatter, Result};

/// Display fatty acid
pub enum FattyAcid {
    Common(Common),
    System(System),
}

impl Display for FattyAcid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            FattyAcid::Common(Common::Delta(delta)) => Display::fmt(delta, f),
            FattyAcid::System(system) => Display::fmt(system, f),
        }
    }
}

/// Display common options
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Options {
    pub kind: Kind,
    pub bounds: Elision,
    pub isomerism: Elision,
}

/// Elision
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Elision {
    Explicit,
    #[default]
    Implicit,
}

/// Display kind
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Kind {
    #[default]
    Delta,
    System,
}

mod common {
    use super::*;

    /// Display common
    pub enum Common {
        /// Carbons are counted from the carboxylic acid end.
        Delta(Delta),
        // /// Carbons are counted from the methyl end.
        // Omega (Omega ),
    }

    /// Display delta
    pub struct Delta {
        pub carbons: u8,
        pub unsaturated: Vec<(usize, Option<Unsaturated>)>,
        pub options: Options,
    }

    impl Display for Delta {
        fn fmt(&self, f: &mut Formatter) -> Result {
            Display::fmt(&self.carbons, f)?;
            f.write_str(":")?;
            Display::fmt(&self.unsaturated.len(), f)?;
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

    impl Display for CommonUnsaturated {
        fn fmt(&self, f: &mut Formatter) -> Result {
            match self.unsaturated.unsaturation {
                None => f.write_str("?")?,
                Some(Unsaturation::Double) => {
                    if self.options.bounds == Elision::Explicit {
                        f.write_str("=")?;
                    }
                }
                Some(Unsaturation::Triple) => f.write_str("≡")?,
            };
            Display::fmt(&self.index, f)?;
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
}

mod system {
    use super::*;

    /// Display system
    pub struct System {
        pub carbons: u8,
        pub unsaturated: Vec<(usize, Option<Unsaturated>)>,
    }

    impl Display for System {
        fn fmt(&self, f: &mut Formatter) -> Result {
            f.write_str("c")?;
            Display::fmt(&self.carbons, f)?;
            f.write_str("u")?;
            Display::fmt(&self.unsaturated.len(), f)?;
            for &(index, unsaturated) in &self.unsaturated {
                Display::fmt(&SystemUnsaturated::new(index, unsaturated), f)?;
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

    impl Display for SystemUnsaturated {
        fn fmt(&self, f: &mut Formatter) -> Result {
            match self
                .unsaturated
                .and_then(|unsaturated| unsaturated.unsaturation)
            {
                None => f.write_str("u")?,
                Some(Unsaturation::Double) => f.write_str("d")?,
                Some(Unsaturation::Triple) => f.write_str("t")?,
            };
            match self
                .unsaturated
                .and_then(|unsaturated| unsaturated.isomerism)
            {
                None => f.write_str("i")?,
                Some(Isomerism::Cis) => f.write_str("c")?,
                Some(Isomerism::Trans) => f.write_str("t")?,
            };
            Display::fmt(&self.index, f)
        }
    }
}
