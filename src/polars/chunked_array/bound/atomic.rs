use crate::prelude::*;

impl BoundChunked {
    /// Returns the number of carbon atoms in the bound chunked array.
    ///
    /// The number of carbon atoms is calculated as the number of bonds plus
    /// one.
    pub fn carbons(&self) -> u8 {
        self.len() + 1
    }

    /// Returns the number of hydrogen atoms in the bound chunked array.
    ///
    /// The number of hydrogen atoms is calculated using the formula `2 * C - 2
    /// * U`, where `C` is the number of carbon atoms and `U` is the number of
    /// unsaturations.
    pub fn hydrogens(&self) -> u8 {
        2 * self.carbons() - 2 * self.unsaturation()
    }
}
