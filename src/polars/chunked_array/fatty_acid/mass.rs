use crate::prelude::*;

impl Mass for &FattyAcidChunked {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        self.triple_bound_list.mass(adduct)
    }
}

impl Mass for Rco<&FattyAcidChunked> {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        Rco(&self.0.triple_bound_list).mass(adduct)
    }
}

impl Mass for Rcoo<&FattyAcidChunked> {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        Rcoo(&self.0.triple_bound_list).mass(adduct)
    }
}

impl Mass for Rcooh<&FattyAcidChunked> {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        Rcooh(&self.0.triple_bound_list).mass(adduct)
    }
}

impl Mass for Rcooch3<&FattyAcidChunked> {
    type Output = f64;

    fn mass(self, adduct: Option<f64>) -> f64 {
        Rcooch3(&self.0.triple_bound_list).mass(adduct)
    }
}

impl FattyAcidChunked {
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
