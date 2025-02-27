//! Represents different types of chemical bonds with optional isomerism.

use self::identifiers::{D, DC, DT, S, T, TC, TT};
use polars::prelude::*;
use polars_arrow::array::Utf8ViewArray;
use std::sync::LazyLock;

pub const IDENTIFIERS: [&str; 7] = [S, D, DC, DT, T, TC, TT];

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Bound {
    #[default]
    Single = 1,
    Double(Option<Isomerism>) = 2,
    Triple(Option<Isomerism>) = 3,
}

/// Represents a bound with various types and associated constants.
impl Bound {
    /// Constant for single bound type.
    pub const S: &str = S;
    /// Constant for double bound type.
    pub const D: &str = D;
    /// Constant for double conjugated bound type.
    pub const DC: &str = DC;
    /// Constant for double trans bound type.
    pub const DT: &str = DT;
    /// Constant for triple bound type.
    pub const T: &str = T;
    /// Constant for triple conjugated bound type.
    pub const TC: &str = TC;
    /// Constant for triple trans bound type.
    pub const TT: &str = TT;

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
    pub fn unsaturated(&self) -> bool {
        match self {
            Self::Single => false,
            _ => true,
        }
    }

    /// Returns the level of unsaturation of the bound.
    ///
    /// # Returns
    ///
    /// * `0` for single bound.
    /// * `1` for double bound.
    /// * `2` for triple bound.
    pub fn unsaturation(&self) -> u8 {
        match self {
            Self::Single => 0,
            Self::Double(_) => 1,
            Self::Triple(_) => 2,
        }
    }
}

impl From<Bound> for &'static str {
    fn from(value: Bound) -> Self {
        match value {
            Bound::Single => S,
            Bound::Double(None) => D,
            Bound::Double(Some(Isomerism::Cis)) => DC,
            Bound::Double(Some(Isomerism::Trans)) => DT,
            Bound::Triple(None) => T,
            Bound::Triple(Some(Isomerism::Cis)) => TC,
            Bound::Triple(Some(Isomerism::Trans)) => TT,
        }
    }
}

impl<'a> TryFrom<&'a str> for Bound {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            S => Ok(Self::Single),
            D => Ok(Self::Double(None)),
            DC => Ok(Self::Double(Some(Isomerism::Cis))),
            DT => Ok(Self::Double(Some(Isomerism::Trans))),
            T => Ok(Self::Triple(None)),
            TC => Ok(Self::Triple(Some(Isomerism::Cis))),
            TT => Ok(Self::Triple(Some(Isomerism::Trans))),
            value => Err(value),
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

pub mod identifiers {
    pub const S: &str = "S";
    pub const D: &str = "D";
    pub const DC: &str = "DC";
    pub const DT: &str = "DT";
    pub const T: &str = "T";
    pub const TC: &str = "TC";
    pub const TT: &str = "TT";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_bound() {
        let single_bound = Bound::new("S");
        assert_eq!(single_bound.unsaturated(), false);
        assert_eq!(single_bound.unsaturation(), 0);
    }

    #[test]
    fn test_double_bound() {
        let double_bound = Bound::new("D");
        assert_eq!(double_bound.unsaturated(), true);
        assert_eq!(double_bound.unsaturation(), 1);
    }

    #[test]
    fn test_double_bound_cis() {
        let double_bound_cis = Bound::new("DC");
        assert_eq!(double_bound_cis.unsaturated(), true);
        assert_eq!(double_bound_cis.unsaturation(), 1);
    }

    #[test]
    fn test_double_bound_trans() {
        let double_bound_trans = Bound::new("DT");
        assert_eq!(double_bound_trans.unsaturated(), true);
        assert_eq!(double_bound_trans.unsaturation(), 1);
    }

    #[test]
    fn test_triple_bound() {
        let triple_bound = Bound::new("T");
        assert_eq!(triple_bound.unsaturated(), true);
        assert_eq!(triple_bound.unsaturation(), 2);
    }

    #[test]
    fn test_triple_bound_cis() {
        let triple_bound_cis = Bound::new("TC");
        assert_eq!(triple_bound_cis.unsaturated(), true);
        assert_eq!(triple_bound_cis.unsaturation(), 2);
    }

    #[test]
    fn test_triple_bound_trans() {
        let triple_bound_trans = Bound::new("TT");
        assert_eq!(triple_bound_trans.unsaturated(), true);
        assert_eq!(triple_bound_trans.unsaturation(), 2);
    }

    #[test]
    #[should_panic(expected = "unexpected bound identifier")]
    fn test_invalid_bound() {
        Bound::new("X");
    }
}
