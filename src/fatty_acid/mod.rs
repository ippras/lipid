pub use self::ext::FattyAcidExt;

use serde::{Deserialize, Serialize};

pub macro fatty_acid($c:expr $(; $($i:expr),*)*) {{
    assert!($c > 0);
    #[allow(unused_mut)]
    let mut fatty_acid = FattyAcid::new($c);
    let mut _count = 0;
    $(
        _count += 1;
        $(
            assert!($i != 0);
            assert!($i < $c);
            let i = ($i as i8);
            let unsaturated = Unsaturated {
                unsaturation: Unsaturation::try_from(_count).ok(),
                index: (i != 0).then_some(i.abs() as _) ,
                isomerism: Isomerism::try_from(i).ok(),
            };
            fatty_acid.unsaturated.push(unsaturated);
        )*
    )*
    fatty_acid
}}

/// Fatty acid
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct FattyAcid {
    pub carbons: u8,
    pub unsaturated: Vec<Unsaturated>,
}

impl FattyAcid {
    pub const fn new(carbons: u8) -> Self {
        Self {
            carbons,
            unsaturated: Vec::new(),
        }
    }

    /// Unsaturation
    pub fn unsaturation(&self) -> u8 {
        self.unsaturated.iter().fold(0, |sum, bound| {
            match bound.unsaturation.unwrap_or_default() {
                Unsaturation::One => sum + 1,
                Unsaturation::Two => sum + 2,
            }
        })
    }
}

// Field::new("Carbons".into(), DataType::UInt8),
// Field::new(
//     "Unsaturated".into(),
//     DataType::Struct(vec![
//         Field::new("Index".into(), DataType::List(Box::new(DataType::UInt8))),
//         Field::new("Isomerism".into(), DataType::List(Box::new(DataType::Int8))),
//         Field::new(
//             "Unsaturation".into(),
//             DataType::List(Box::new(DataType::UInt8)),
//         ),
//     ]),
// ),

/// Unsaturated
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Unsaturated {
    pub index: Option<u8>,
    pub isomerism: Option<Isomerism>,
    pub unsaturation: Option<Unsaturation>,
}

/// Isomerism
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum Isomerism {
    #[default]
    Cis = 1,
    Trans = -1,
}

impl From<Isomerism> for i8 {
    fn from(value: Isomerism) -> Self {
        match value {
            Isomerism::Cis => 1,
            Isomerism::Trans => -1,
        }
    }
}

impl TryFrom<i8> for Isomerism {
    type Error = i8;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value.is_positive() {
            Ok(Self::Cis)
        } else if value.is_negative() {
            Ok(Self::Trans)
        } else {
            Err(value)
        }
    }
}

/// Unsaturation
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum Unsaturation {
    #[default]
    One = 1,
    Two = 2,
}

impl TryFrom<u8> for Unsaturation {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err(value),
        }
    }
}

/// Fatty acid kind
#[derive(Clone, Copy, Debug, Default)]
pub enum Kind {
    /// Fatty acid
    ///
    /// `RCOOH`
    #[default]
    Rcooh,
    /// Fatty acid methyl ester
    ///
    /// `RCOOH -H +CH3 => RCOOCH3`
    Rcooch3,
    /// Fatty acid [RCOO]-
    ///
    /// `RCOOH -H => [RCOO]-`
    Rcoo,
    /// Fatty acid [RCO]+
    ///
    /// `RCOOH -OH => [RCO]+`
    Rco,
}

pub mod r#const;
pub mod display;
pub mod ext;
pub mod mass;
