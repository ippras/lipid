use crate::prelude::*;
use atom::isotopes::*;

const C: f64 = C::Twelve.relative_atomic_mass().value;
const H: f64 = H::One.relative_atomic_mass().value;
const O: f64 = O::Sixteen.relative_atomic_mass().value;

impl Mass for &BoundChunked {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        self.carbons() as f64 * C + self.hydrogens() as f64 * H + 2.0 * O + adduct.unwrap_or(0.0)
    }
}

impl Mass for Rco<&BoundChunked> {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        self.0.mass(adduct) - H - O
    }
}

impl Mass for Rcoo<&BoundChunked> {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        self.0.mass(adduct) - H
    }
}

impl Mass for Rcooh<&BoundChunked> {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        self.0.mass(adduct)
    }
}

impl Mass for Rcooch3<&BoundChunked> {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        self.0.mass(adduct) + 2.0 * H + C
    }
}

impl BoundChunked {
    pub fn rco(&self) -> Rco<&Self> {
        Rco(self)
    }

    pub fn rcoo(&self) -> Rcoo<&Self> {
        Rcoo(self)
    }

    pub fn rcooh(&self) -> Rcooh<&Self> {
        Rcooh(self)
    }

    pub fn rcooch3(&self) -> Rcooch3<&Self> {
        Rcooch3(self)
    }
}
