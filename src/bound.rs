//! Represents different types of chemical bonds with optional isomerism.

use std::fmt::{self, Display, Formatter};

/// Bound.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bound(pub(crate) Option<Explicit>);

impl Bound {
    /// Constant for unknown bound type.
    pub const B: Self = Self(None);
    /// Constant for single bound type.
    pub const S: Self = Self(Some(Explicit::S));
    /// Constant for double cis bound type.
    pub const DC: Self = Self(Some(Explicit::DC));
    /// Constant for double trans bound type.
    pub const DT: Self = Self(Some(Explicit::DT));
    /// Constant for double bound type.
    pub const D: Self = Self(Some(Explicit::D));
    /// Constant for triple cis bound type.
    pub const TC: Self = Self(Some(Explicit::TC));
    /// Constant for triple trans bound type.
    pub const TT: Self = Self(Some(Explicit::TT));
    /// Constant for triple bound type.
    pub const T: Self = Self(Some(Explicit::T));
    /// Constant for unknown cis bound type.
    pub const UC: Self = Self(Some(Explicit::UC));
    /// Constant for unknown trans bound type.
    pub const UT: Self = Self(Some(Explicit::UT));
    /// Constant for unknown bound type.
    pub const U: Self = Self(Some(Explicit::U));
}

impl Display for Bound {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0 {
            None if f.alternate() => f.write_str("B")?,
            None => f.write_str("b")?,
            Some(Explicit::Saturated) if f.alternate() => f.write_str("S")?,
            Some(Explicit::Saturated) => f.write_str("s")?,
            Some(Explicit::Unsaturated(unsaturated)) => {
                match unsaturated.unsaturation {
                    None if f.alternate() => f.write_str("U")?,
                    None => f.write_str("u")?,
                    Some(Unsaturation::Double) if f.alternate() => f.write_str("D")?,
                    Some(Unsaturation::Double) => f.write_str("d")?,
                    Some(Unsaturation::Triple) if f.alternate() => f.write_str("T")?,
                    Some(Unsaturation::Triple) => f.write_str("t")?,
                }
                match unsaturated.isomerism {
                    None => {}
                    Some(Isomerism::Cis) => f.write_str("c")?,
                    Some(Isomerism::Trans) => f.write_str("t")?,
                }
            }
        };
        Ok(())
    }
}

/// Represents a bound with various types and associated constants.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Explicit {
    #[default]
    Saturated,
    Unsaturated(Unsaturated),
}

impl Explicit {
    /// Constant for single bound type.
    pub const S: Self = Self::Saturated;
    /// Constant for double cis bound type.
    pub const DC: Self = Self::Unsaturated(Unsaturated::DC);
    /// Constant for double trans bound type.
    pub const DT: Self = Self::Unsaturated(Unsaturated::DT);
    /// Constant for double bound type.
    pub const D: Self = Self::Unsaturated(Unsaturated::D);
    /// Constant for triple cis bound type.
    pub const TC: Self = Self::Unsaturated(Unsaturated::TC);
    /// Constant for triple trans bound type.
    pub const TT: Self = Self::Unsaturated(Unsaturated::TT);
    /// Constant for triple bound type.
    pub const T: Self = Self::Unsaturated(Unsaturated::T);
    /// Constant for unknown cis bound type.
    pub const UC: Self = Self::Unsaturated(Unsaturated::UC);
    /// Constant for unknown trans bound type.
    pub const UT: Self = Self::Unsaturated(Unsaturated::UT);
    /// Constant for unknown bound type.
    pub const U: Self = Self::Unsaturated(Unsaturated::U);

    pub fn as_saturated(self) -> Option<Saturated> {
        match self {
            Self::Saturated => Some(Saturated),
            _ => None,
        }
    }

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
            Self::Unsaturated(Unsaturated {
                unsaturation: Some(unsaturation),
                ..
            }) => unsaturation == Unsaturation::Double,
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
            Self::Unsaturated(Unsaturated {
                unsaturation: Some(unsaturation),
                ..
            }) => unsaturation == Unsaturation::Triple,
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
            Self::Unsaturated(Unsaturated {
                isomerism: Some(isomerism),
                ..
            }) => isomerism == Isomerism::Cis,
            _ => false,
        }
    }

    pub fn is_trans(self) -> bool {
        match self {
            Self::Unsaturated(Unsaturated {
                isomerism: Some(isomerism),
                ..
            }) => isomerism == Isomerism::Trans,
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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Saturated;

/// Represents an unsaturated bond with optional isomerism.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Unsaturated {
    pub unsaturation: Option<Unsaturation>,
    pub isomerism: Option<Isomerism>,
}

impl Unsaturated {
    pub const D: Self = Self::new(Some(Unsaturation::Double), None);
    pub const DC: Self = Self::new(Some(Unsaturation::Double), Some(Isomerism::Cis));
    pub const DT: Self = Self::new(Some(Unsaturation::Double), Some(Isomerism::Trans));
    pub const T: Self = Self::new(Some(Unsaturation::Triple), None);
    pub const TC: Self = Self::new(Some(Unsaturation::Triple), Some(Isomerism::Cis));
    pub const TT: Self = Self::new(Some(Unsaturation::Triple), Some(Isomerism::Trans));
    pub const U: Self = Self::new(None, None);
    pub const UC: Self = Self::new(None, Some(Isomerism::Cis));
    pub const UT: Self = Self::new(None, Some(Isomerism::Trans));

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

/// Represents the level of unsaturation.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Unsaturation {
    #[default]
    Double = 1,
    Triple = 2,
}

/// Represents the type of isomerism.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
