use crate::prelude::*;
use atom::isotopes::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;
const O: f64 = O::Sixteen.relative_atomic_mass().value;

impl BoundChunked {
    pub fn mass(&self, adduct: Option<f64>) -> f64 {
        self.carbons() as f64 * C + self.hydrogens() as f64 * H + 2.0 * O + adduct.unwrap_or(0.0)
    }
}

impl Rco<&BoundChunked> {
    pub fn mass(self, adduct: Option<f64>) -> f64 {
        self.0.mass(adduct) - H - O
    }
}

impl Rcoo<&BoundChunked> {
    pub fn mass(self, adduct: Option<f64>) -> f64 {
        self.0.mass(adduct) - H
    }
}

impl Rcooh<&BoundChunked> {
    pub fn mass(self, adduct: Option<f64>) -> f64 {
        self.0.mass(adduct)
    }
}

impl Rcooch3<&BoundChunked> {
    pub fn mass(self, adduct: Option<f64>) -> f64 {
        self.0.mass(adduct) + 2.0 * H + C
    }
}
