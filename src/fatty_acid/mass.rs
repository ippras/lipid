use super::{FattyAcidExt, Kind};
use atom::isotopes::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;
const O: f64 = O::Sixteen.relative_atomic_mass().value;

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
