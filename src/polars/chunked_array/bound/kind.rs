use crate::prelude::*;

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
