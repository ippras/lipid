/// Unsaturated bound
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Unsaturated<T, U, V> {
    pub index: T,
    pub triple: U,
    pub parity: V,
}

impl Unsaturated<Option<u8>, Option<bool>, Option<bool>> {
    pub fn new(index: Option<u8>, triple: Option<bool>, parity: Option<bool>) -> Self {
        Self {
            index,
            triple,
            parity,
        }
    }
}
