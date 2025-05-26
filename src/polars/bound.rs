//! Represents different types of chemical bonds with optional isomerism.

use self::identifiers::logical::{D, DC, DT, S, T, TC, TT, U, UC, UT};
use crate::prelude::*;
use polars::prelude::*;
use polars_arrow::array::Utf8ViewArray;
use std::sync::LazyLock;

/// Index field name.
pub const INDEX: &str = "Index";
/// Identifier field name.
pub const BOUND: &str = "Bound";
/// Array of bond identifiers.
pub const IDENTIFIERS: [&str; 10] = [S, DC, DT, D, TC, TT, T, UC, UT, U];

pub const MAP: LazyLock<Arc<RevMapping>> = LazyLock::new(|| {
    Arc::new(RevMapping::build_local(Utf8ViewArray::from_slice_values(
        IDENTIFIERS,
    )))
});

impl Bound {
    pub fn categorical(&self) -> Option<u32> {
        Some(self.0?.categorical())
    }
}

impl Explicit {
    /// Creates a new [`Bound`] instance from the given identifier string.
    ///
    /// # Arguments
    ///
    /// * `id` - A string slice that holds the identifier for the bound.
    ///
    /// # Panics
    ///
    /// Panics if the identifier is not recognized.
    pub fn new(id: &str) -> Self {
        id.try_into().expect(&format!(
            "unexpected bound identifier: {id}; expected: {IDENTIFIERS:?}",
        ))
    }

    pub fn categorical(&self) -> u32 {
        match *self {
            Self::S => physical::S,
            Self::DC => physical::DC,
            Self::DT => physical::DT,
            Self::D => physical::D,
            Self::TC => physical::TC,
            Self::TT => physical::TT,
            Self::T => physical::T,
            Self::UC => physical::UC,
            Self::UT => physical::UT,
            Self::U => physical::U,
        }
    }
}

impl From<Explicit> for &'static str {
    fn from(value: Explicit) -> Self {
        match value {
            Explicit::Saturated => S,
            Explicit::Unsaturated(unsaturated) => unsaturated.into(),
        }
    }
}

impl<'a> TryFrom<&'a str> for Explicit {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            S => Ok(Self::Saturated),
            value => Ok(Self::Unsaturated(value.try_into()?)),
        }
    }
}

impl Unsaturated {
    pub fn new_unchecked(id: &str) -> Self {
        id.try_into().expect(&format!(
            "unexpected unsaturated identifier: {id}; expected: {IDENTIFIERS:?}",
        ))
    }
}

impl From<Unsaturated> for &'static str {
    fn from(value: Unsaturated) -> Self {
        match value {
            Unsaturated::D => D,
            Unsaturated::DC => DC,
            Unsaturated::DT => DT,
            Unsaturated::T => T,
            Unsaturated::TC => TC,
            Unsaturated::TT => TT,
            Unsaturated::U => U,
            Unsaturated::UC => UC,
            Unsaturated::UT => UT,
        }
    }
}

impl<'a> TryFrom<&'a str> for Unsaturated {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            D => Ok(Unsaturated::D),
            DC => Ok(Unsaturated::DC),
            DT => Ok(Unsaturated::DT),
            T => Ok(Unsaturated::T),
            TC => Ok(Unsaturated::TC),
            TT => Ok(Unsaturated::TT),
            U => Ok(Unsaturated::U),
            UC => Ok(Unsaturated::UC),
            UT => Ok(Unsaturated::UT),
            value => Err(value),
        }
    }
}

pub mod identifiers {
    pub mod physical {
        pub const S: u32 = 0;
        pub const DC: u32 = 1;
        pub const DT: u32 = 2;
        pub const D: u32 = 3;
        pub const TC: u32 = 4;
        pub const TT: u32 = 5;
        pub const T: u32 = 6;
        pub const UC: u32 = 7;
        pub const UT: u32 = 8;
        pub const U: u32 = 9;

        pub const CIS: u32 = 1;
        pub const TRANS: u32 = 2;
    }

    pub mod logical {
        pub const B: &str = "B";
        pub const S: &str = "S";
        pub const DC: &str = "DC";
        pub const DT: &str = "DT";
        pub const D: &str = "D";
        pub const TC: &str = "TC";
        pub const TT: &str = "TT";
        pub const T: &str = "T";
        pub const UC: &str = "UC";
        pub const UT: &str = "UT";
        pub const U: &str = "U";
    }
}
