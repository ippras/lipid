use super::{FattyAcid, Kind};
use atom::isotopes::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;
const O: f64 = O::Sixteen.relative_atomic_mass().value;

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
}

/// Mass
pub trait Mass: FattyAcidExt {
    /// Mass
    fn mass(&self, kind: Kind) -> f64;
}

impl<T: FattyAcidExt<Output = u8>> Mass for T {
    fn mass(&self, kind: Kind) -> f64 {
        let mut c = self.carbons();
        let mut h = self.hydrogens();
        let mut o = 2;
        match kind {
            Kind::Rcooh => {}
            Kind::Rcooch3 => {
                c += 1;
                h += 2;
            }
            Kind::Rcoo => h -= 1,
            Kind::Rco => {
                h -= 1;
                o -= 1;
            }
        }
        c as f64 * C + h as f64 * H + o as f64 * O
    }
}
