/// Fatty acid
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FattyAcid<T = u8, U = u8, V = Vec<Index>> {
    pub carbon: T,
    pub unsaturated: U,
    pub indices: V,
}

/// Index
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Index<T = Option<i8>, U = Option<bool>, V = Option<bool>> {
    pub index: T,
    pub triple: U,
    pub parity: V,
}

pub mod display;
