/// Fatty acid
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FattyAcid<T = u8, U = Vec<Unsaturated>> {
    pub carbon: T,
    pub unsaturated: U,
}

/// Unsaturated bound
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Unsaturated<T = Option<u8>, U = Option<bool>, V = Option<bool>> {
    pub index: T,
    pub triple: U,
    pub parity: V,
}

pub mod display;
