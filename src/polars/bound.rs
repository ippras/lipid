use polars::prelude::*;
use polars_arrow::array::Utf8ViewArray;
use std::sync::LazyLock;

pub const LETTERS: [&str; 5] = ["s", "dc", "dt", "tc", "tt"];
pub const DATA_TYPE: LazyLock<DataType> = LazyLock::new(|| {
    let categories = Utf8ViewArray::from_slice_values(LETTERS);
    create_enum_dtype(categories)
});

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Bound {
    #[default]
    Single = 1,
    Double(Isomerism) = 2,
    Triple(Isomerism) = 3,
}

impl From<Bound> for &'static str {
    fn from(value: Bound) -> Self {
        match value {
            Bound::Single => "s",
            Bound::Double(Isomerism::Cis) => "dc",
            Bound::Double(Isomerism::Trans) => "dt",
            Bound::Triple(Isomerism::Cis) => "tc",
            Bound::Triple(Isomerism::Trans) => "tt",
        }
    }
}

impl<'a> TryFrom<&'a str> for Bound {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "s" => Ok(Self::Single),
            "dc" => Ok(Self::Double(Isomerism::Cis)),
            "dt" => Ok(Self::Double(Isomerism::Trans)),
            "tc" => Ok(Self::Triple(Isomerism::Cis)),
            "tt" => Ok(Self::Triple(Isomerism::Trans)),
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
