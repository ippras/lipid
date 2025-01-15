use super::FattyAcid;

/// Extension methods for [`FattyAcid`]
pub trait FattyAcidExt {
    type Output;

    /// Carbons
    fn carbons(&self) -> Self::Output;

    /// Hydrogens
    ///
    /// `H = 2C - 2U`
    fn hydrogens(&self) -> Self::Output;

    /// Bounds
    ///
    /// The number of bonds.
    fn bounds(&self) -> Self::Output;

    /// Saturated
    ///
    /// The number of saturated bonds.
    fn saturated(&self) -> Self::Output;

    /// Unsaturated
    ///
    /// The number of unsaturated bonds.
    fn unsaturated(&self) -> Self::Output;

    /// Is saturated
    fn is_saturated(&self) -> bool;

    /// Is unsaturated
    fn is_unsaturated(&self) -> bool {
        !self.is_saturated()
    }
}

impl FattyAcidExt for FattyAcid {
    type Output = u8;

    fn carbons(&self) -> Self::Output {
        self.carbons
    }

    fn hydrogens(&self) -> Self::Output {
        2 * self.carbons - 2 * self.unsaturated()
    }

    fn bounds(&self) -> Self::Output {
        self.carbons.saturating_sub(1)
    }

    fn saturated(&self) -> Self::Output {
        self.bounds() - self.unsaturated()
    }

    fn unsaturated(&self) -> Self::Output {
        self.unsaturated
            .iter()
            .filter(|unsaturated| unsaturated.unsaturation.is_some())
            .count() as _
    }

    fn is_saturated(&self) -> bool {
        self.unsaturated() == 0
    }
}
