use crate::prelude::*;

impl BoundChunked {
    #[inline]
    pub fn equivalent_carbon_number(&self) -> u8 {
        self.carbons() - 2 * self.unsaturation()
    }
}
