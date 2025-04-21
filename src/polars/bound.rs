//! Represents different types of chemical bonds with optional isomerism.

use self::identifiers::{D, DC, DT, S, T, TC, TT, U, UC, UT};
use crate::prelude::*;
use polars::prelude::*;
use polars_arrow::array::Utf8ViewArray;
use std::sync::LazyLock;

/// Index field name.
pub const INDEX: &str = "Index";
/// Identifier field name.
pub const IDENTIFIER: &str = "Identifier";
/// Array of bond identifiers.
pub const IDENTIFIERS: [&str; 10] = [S, U, UC, UT, D, DC, DT, T, TC, TT];

pub const MAP: LazyLock<Arc<RevMapping>> = LazyLock::new(|| {
    Arc::new(RevMapping::build_local(Utf8ViewArray::from_slice_values(
        IDENTIFIERS,
    )))
});

impl Bound {
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
}

impl From<Bound> for &'static str {
    fn from(value: Bound) -> Self {
        match value {
            Bound::Saturated => S,
            Bound::Unsaturated(unsaturated) => unsaturated.into(),
        }
    }
}

impl<'a> TryFrom<&'a str> for Bound {
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
    pub const S: &str = "S";
    pub const D: &str = "D";
    pub const DC: &str = "DC";
    pub const DT: &str = "DT";
    pub const T: &str = "T";
    pub const TC: &str = "TC";
    pub const TT: &str = "TT";
    pub const U: &str = "U";
    pub const UC: &str = "UC";
    pub const UT: &str = "UT";
}
