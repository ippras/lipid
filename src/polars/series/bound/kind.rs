use super::BoundSeries;
use crate::prelude::*;

impl BoundSeries {
    pub fn rco(&self) -> Rco<&BoundSeries> {
        Rco(self)
    }

    pub fn rcoo(&self) -> Rcoo<&BoundSeries> {
        Rcoo(self)
    }

    pub fn rcooh(&self) -> Rcooh<&BoundSeries> {
        Rcooh(self)
    }

    pub fn rcooch3(&self) -> Rcooch3<&BoundSeries> {
        Rcooch3(self)
    }
}
