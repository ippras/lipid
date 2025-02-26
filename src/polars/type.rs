use self::identifiers::{S, U};
use polars::prelude::*;
use polars_arrow::array::Utf8ViewArray;
use std::sync::LazyLock;

pub const IDENTIFIERS: [&str; 2] = [S, U];

/// Type
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Type {
    Saturated,
    Unsaturated,
}

impl Type {
    pub const S: &str = S;
    pub const U: &str = U;

    pub const DATA_TYPE: LazyLock<DataType> = LazyLock::new(|| {
        let categories = Utf8ViewArray::from_slice_values(IDENTIFIERS);
        create_enum_dtype(categories)
    });
}

impl From<Type> for &'static str {
    fn from(value: Type) -> Self {
        match value {
            Type::Saturated => S,
            Type::Unsaturated => U,
        }
    }
}

impl<'a> TryFrom<&'a str> for Type {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            S => Ok(Self::Saturated),
            U => Ok(Self::Unsaturated),
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
    pub const U: &str = "U";
}
