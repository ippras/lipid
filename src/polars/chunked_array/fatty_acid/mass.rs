use crate::prelude::*;

impl Mass for &FattyAcidChunked {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        self.bound.mass(adduct)
    }
}

impl Mass for Rco<&FattyAcidChunked> {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        Rco(&self.0.bound).mass(adduct)
    }
}

impl Mass for Rcoo<&FattyAcidChunked> {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        Rcoo(&self.0.bound).mass(adduct)
    }
}

impl Mass for Rcooh<&FattyAcidChunked> {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        Rcooh(&self.0.bound).mass(adduct)
    }
}

impl Mass for Rcooch3<&FattyAcidChunked> {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        Rcooch3(&self.0.bound).mass(adduct)
    }
}
