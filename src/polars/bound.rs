//! Represents different types of chemical bonds with optional isomerism.

use self::identifiers::{D, DC, DT, S, T, TC, TT, U, UC, UT};
use polars::prelude::*;
use polars_arrow::array::Utf8ViewArray;
use std::sync::LazyLock;

/// Array of bond identifiers.
pub const IDENTIFIERS: [&str; 10] = [S, D, DC, DT, T, TC, TT, U, UC, UT];

/// The bond data type.
pub const BOUND_DATA_TYPE: LazyLock<DataType> = LazyLock::new(|| {
    let categories = Utf8ViewArray::from_slice_values(IDENTIFIERS);
    create_enum_dtype(categories)
});

/// Represents a bound with various types and associated constants.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Bound {
    #[default]
    Saturated,
    Unsaturated(Unsaturated),
}

impl Bound {
    /// Constant for single bound type.
    pub const S: Self = Self::Saturated;
    /// Constant for double bound type.
    pub const D: Self = Self::Unsaturated(Unsaturated::D);
    /// Constant for double cis bound type.
    pub const DC: Self = Self::Unsaturated(Unsaturated::DC);
    /// Constant for double trans bound type.
    pub const DT: Self = Self::Unsaturated(Unsaturated::DT);
    /// Constant for triple bound type.
    pub const T: Self = Self::Unsaturated(Unsaturated::T);
    /// Constant for triple cis bound type.
    pub const TC: Self = Self::Unsaturated(Unsaturated::TC);
    /// Constant for triple trans bound type.
    pub const TT: Self = Self::Unsaturated(Unsaturated::TT);
    /// Constant for unknown bound type.
    pub const U: Self = Self::Unsaturated(Unsaturated::T);
    /// Constant for unknown cis bound type.
    pub const UC: Self = Self::Unsaturated(Unsaturated::UC);
    /// Constant for unknown trans bound type.
    pub const UT: Self = Self::Unsaturated(Unsaturated::UT);

    /// Lazy static initialization for the data type associated with the bound.
    pub const DATA_TYPE: LazyLock<DataType> = BOUND_DATA_TYPE;

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

    // pub fn data_type() -> DataType {
    //     BOUND_DATA_TYPE
    // }

    /// Converts the bond to an optional unsaturated bond.
    ///
    /// # Returns
    ///
    /// * [`Some`] if the bond is unsaturated, otherwise [`None`].
    pub fn as_unsaturated(self) -> Option<Unsaturated> {
        match self {
            Self::Unsaturated(unsaturated) => Some(unsaturated),
            _ => None,
        }
    }

    /// Checks if the bound is saturated.
    ///
    /// # Returns
    ///
    /// * [`true`] if the bound is saturated, otherwise [`false`].
    pub fn is_saturated(self) -> bool {
        match self {
            Self::Saturated => true,
            _ => false,
        }
    }

    /// Checks if the bound is double.
    ///
    /// # Returns
    ///
    /// * [`true`] if the bound is double, otherwise [`false`].
    pub fn is_double(self) -> bool {
        match self {
            Self::D | Self::DC | Self::DT => true,
            _ => false,
        }
    }

    /// Checks if the bound is triple.
    ///
    /// # Returns
    ///
    /// * [`true`] if the bound is triple, otherwise [`false`].
    pub fn is_triple(self) -> bool {
        match self {
            Self::T | Self::TC | Self::TT => true,
            _ => false,
        }
    }

    /// Checks if the bound is unsaturated.
    ///
    /// # Returns
    ///
    /// * [`true`] if the bound is unsaturated, otherwise [`false`].
    pub fn is_unsaturated(self) -> bool {
        match self {
            Self::Unsaturated(_) => true,
            _ => false,
        }
    }

    pub fn is_cis(self) -> bool {
        match self {
            Self::DC | Self::TC | Self::UC => true,
            _ => false,
        }
    }

    pub fn is_trans(self) -> bool {
        match self {
            Self::DT | Self::TT | Self::UT => true,
            _ => false,
        }
    }

    /// Returns the level of unsaturation of the bound.
    ///
    /// # Returns
    ///
    /// * `0` for single (saturated) bound.
    /// * `1` for double bound.
    /// * `2` for triple bound.
    pub fn unsaturation(self) -> Option<u8> {
        match self {
            Self::Saturated => Some(0),
            Self::Unsaturated(unsaturated) => unsaturated.unsaturation(),
        }
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

/// Represents an unsaturated bond with optional isomerism.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unsaturated {
    pub unsaturation: Option<Unsaturation>,
    pub isomerism: Option<Isomerism>,
}

impl Unsaturated {
    const D: Self = Self::new(Some(Unsaturation::Double), None);
    const DC: Self = Self::new(Some(Unsaturation::Double), Some(Isomerism::Cis));
    const DT: Self = Self::new(Some(Unsaturation::Double), Some(Isomerism::Trans));
    const T: Self = Self::new(Some(Unsaturation::Triple), None);
    const TC: Self = Self::new(Some(Unsaturation::Triple), Some(Isomerism::Cis));
    const TT: Self = Self::new(Some(Unsaturation::Triple), Some(Isomerism::Trans));
    const U: Self = Self::new(None, None);
    const UC: Self = Self::new(None, Some(Isomerism::Cis));
    const UT: Self = Self::new(None, Some(Isomerism::Trans));

    /// Creates a new [`Unsaturated`] instance.
    ///
    /// # Arguments
    ///
    /// * `unsaturation` - An optional unsaturation level.
    /// * `isomerism` - An optional isomerism type.
    ///
    /// # Returns
    ///
    /// A new instance of [`Unsaturated`].
    pub const fn new(unsaturation: Option<Unsaturation>, isomerism: Option<Isomerism>) -> Self {
        Self {
            unsaturation,
            isomerism,
        }
    }

    /// Returns the level of unsaturation.
    ///
    /// # Returns
    ///
    /// * `1` for double bond.
    /// * `2` for triple bond.
    pub fn unsaturation(&self) -> Option<u8> {
        match self.unsaturation? {
            Unsaturation::Double => Some(1),
            Unsaturation::Triple => Some(2),
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

/// Represents the level of unsaturation.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Unsaturation {
    #[default]
    Double = 1,
    Triple = 2,
}

/// Represents the type of isomerism.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Isomerism {
    /// Represents cis bond.
    #[default]
    Cis = 1,
    /// Represents trans bond.
    Trans = -1,
}

/// Represents different types of a bond.
///
/// The [`Type`] enum has two variants:
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Type {
    /// Represents saturated bond.
    Saturated,
    /// Represents unsaturated bond.
    Unsaturated,
}

#[inline]
pub fn unsaturation(id: Option<&str>) -> Option<u8> {
    match id? {
        D | DC | DT => Some(1),
        T | TC | TT => Some(2),
        _ => None,
    }
}

/// Checks if the bond is saturated.
///
/// # Arguments
///
/// * `id` - An optional string slice representing the bond identifier.
///
/// # Returns
///
/// * [`true`] if the bond is saturated, otherwise [`false`].
#[inline]
pub fn is_saturated(id: Option<&str>) -> bool {
    id == Some(S)
}

/// Checks if the bond is not unsaturated.
///
/// # Arguments
///
/// * `id` - An optional string slice representing the bond identifier.
///
/// # Returns
///
/// * [`true`] if the bond is not unsaturated, otherwise [`false`].
#[inline]
pub fn is_not_unsaturated(id: Option<&str>) -> bool {
    id.is_none() || id == Some(S)
}

/// Checks if the bond is unsaturated.
///
/// # Arguments
///
/// * `id` - An optional string slice representing the bond identifier.
///
/// # Returns
///
/// * [`true`] if the bond is unsaturated, otherwise [`false`].
#[inline]
pub fn is_unsaturated(id: Option<&str>) -> bool {
    id.is_some() && id != Some(S)
}

/// Checks if the bond is not saturated.
///
/// # Arguments
///
/// * `id` - An optional string slice representing the bond identifier.
///
/// # Returns
///
/// * [`true`] if the bond is not saturated, otherwise [`false`].
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
