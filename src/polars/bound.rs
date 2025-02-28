//! Represents different types of chemical bonds with optional isomerism.

use self::identifiers::{D, DC, DT, S, T, TC, TT, U, UC, UT};
use polars::prelude::*;
use polars_arrow::array::Utf8ViewArray;
use std::sync::LazyLock;

pub const IDENTIFIERS: [&str; 10] = [S, D, DC, DT, T, TC, TT, U, UC, UT];

/// Represents a bound with various types and associated constants.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Bound {
    #[default]
    Saturated,
    Unaturated(Unsaturated),
}

impl Bound {
    /// Constant for single bound type.
    pub const S: &str = S;
    /// Constant for double bound type.
    pub const D: &str = D;
    /// Constant for double cis bound type.
    pub const DC: &str = DC;
    /// Constant for double trans bound type.
    pub const DT: &str = DT;
    /// Constant for triple bound type.
    pub const T: &str = T;
    /// Constant for triple cis bound type.
    pub const TC: &str = TC;
    /// Constant for triple trans bound type.
    pub const TT: &str = TT;
    /// Constant for unknown bound type.
    pub const U: &str = T;
    /// Constant for unknown cis bound type.
    pub const UC: &str = TC;
    /// Constant for unknown trans bound type.
    pub const UT: &str = TT;

    /// Lazy static initialization for the data type associated with the bound.
    pub const DATA_TYPE: LazyLock<DataType> = LazyLock::new(|| {
        let categories = Utf8ViewArray::from_slice_values(IDENTIFIERS);
        create_enum_dtype(categories)
    });

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

    /// Checks if the bound is unsaturated.
    ///
    /// # Returns
    ///
    /// * [`true`] if the bound is unsaturated, otherwise [`false`].
    pub fn is_unsaturated(&self) -> bool {
        match self {
            Self::Saturated => false,
            _ => true,
        }
    }

    /// Returns the level of unsaturation of the bound.
    ///
    /// # Returns
    ///
    /// * `0` for single (saturated) bound.
    /// * `1` for double bound.
    /// * `2` for triple bound.
    pub fn unsaturation(&self) -> Option<u8> {
        match self {
            Self::Saturated => Some(0),
            Self::Unaturated(unsaturated) => unsaturated.unsaturation(),
        }
    }
}

impl From<Bound> for &'static str {
    fn from(value: Bound) -> Self {
        match value {
            Bound::Saturated => S,
            Bound::Unaturated(unsaturated) => unsaturated.into(),
        }
    }
}

impl<'a> TryFrom<&'a str> for Bound {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            S => Ok(Self::Saturated),
            value => Ok(Self::Unaturated(value.try_into()?)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unsaturated {
    pub unsaturation: Option<Unaturation>,
    pub isomerism: Option<Isomerism>,
}

impl Unsaturated {
    const D: Self = Self::new(Some(Unaturation::Double), None);
    const DC: Self = Self::new(Some(Unaturation::Double), Some(Isomerism::Cis));
    const DT: Self = Self::new(Some(Unaturation::Double), Some(Isomerism::Trans));
    const T: Self = Self::new(Some(Unaturation::Triple), None);
    const TC: Self = Self::new(Some(Unaturation::Triple), Some(Isomerism::Cis));
    const TT: Self = Self::new(Some(Unaturation::Triple), Some(Isomerism::Trans));
    const U: Self = Self::new(None, None);
    const UC: Self = Self::new(None, Some(Isomerism::Cis));
    const UT: Self = Self::new(None, Some(Isomerism::Trans));

    pub const fn new(unsaturation: Option<Unaturation>, isomerism: Option<Isomerism>) -> Self {
        Self {
            unsaturation,
            isomerism,
        }
    }

    pub fn unsaturation(&self) -> Option<u8> {
        match self.unsaturation? {
            Unaturation::Double => Some(1),
            Unaturation::Triple => Some(2),
        }
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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Unaturation {
    Double,
    Triple,
}

impl From<Unaturation> for u8 {
    fn from(value: Unaturation) -> Self {
        match value {
            Unaturation::Double => 1,
            Unaturation::Triple => 2,
        }
    }
}

/// Isomerism
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Isomerism {
    #[default]
    Cis = 1,
    Trans = -1,
}

/// Represents different types of a bound.
///
/// The [`Type`] enum has two variants:
/// - `Saturated`: Represents saturated bound.
/// - `Unsaturated`: Represents unsaturated bound.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Type {
    Saturated,
    Unsaturated,
}

/// Filters null and unsaturated
#[inline]
pub fn is_saturated(id: Option<&str>) -> bool {
    id == Some(S)
}

/// Filters unsaturated
#[inline]
pub fn is_not_unsaturated(id: Option<&str>) -> bool {
    id.is_none() || id == Some(S)
}

/// Filters null and saturated
#[inline]
pub fn is_unsaturated(id: Option<&str>) -> bool {
    id.is_some() && id != Some(S)
}

/// Filters saturated
#[inline]
pub fn is_not_saturated(id: Option<&str>) -> bool {
    id != Some(S)
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
