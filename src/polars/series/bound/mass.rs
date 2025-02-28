use super::BoundSeries;
use crate::prelude::*;
use atom::isotopes::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;
const O: f64 = O::Sixteen.relative_atomic_mass().value;

impl BoundSeries {
    pub fn mass(&self, adduct: Option<f64>) -> f64 {
        self.rcooh().mass(adduct)
    }
}

impl Rco<&BoundSeries> {
    pub fn mass(self, adduct: Option<f64>) -> f64 {
        self.0.mass(adduct) - H - O
    }
}

impl Rcoo<&BoundSeries> {
    pub fn mass(self, adduct: Option<f64>) -> f64 {
        self.0.mass(adduct) - H
    }
}

impl Rcooh<&BoundSeries> {
    pub fn mass(self, adduct: Option<f64>) -> f64 {
        let c = self.0.carbons() as f64;
        let h = self.0.hydrogens() as f64;
        let o = 2.0;
        c * C + h * H + o * O + adduct.unwrap_or(0.0)
    }
}

impl Rcooch3<&BoundSeries> {
    pub fn mass(self, adduct: Option<f64>) -> f64 {
        self.0.mass(adduct) + 2.0 * H + C
    }
}
