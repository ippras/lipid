//! Represents different types of chemical bonds with optional isomerism.

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
    pub const U: Self = Self::Unsaturated(Unsaturated::U);
    /// Constant for unknown cis bound type.
    pub const UC: Self = Self::Unsaturated(Unsaturated::UC);
    /// Constant for unknown trans bound type.
    pub const UT: Self = Self::Unsaturated(Unsaturated::UT);

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
