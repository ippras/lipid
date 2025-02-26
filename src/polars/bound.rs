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

impl Bound {
    pub const S: &str = S;
    pub const D: &str = D;
    pub const DC: &str = DC;
    pub const DT: &str = DT;
    pub const T: &str = T;
    pub const TC: &str = TC;
    pub const TT: &str = TT;

    pub const DATA_TYPE: LazyLock<DataType> = LazyLock::new(|| {
        let categories = Utf8ViewArray::from_slice_values(IDENTIFIERS);
        create_enum_dtype(categories)
    });
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
