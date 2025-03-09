use crate::prelude::*;

impl BoundChunked {
    /// Checks if the bounds chunked array contains only saturated bonds.
    pub fn is_saturated(&self) -> bool {
        self.into_iter()
            .all(|bound| bound.is_some_and(Bound::is_saturated))
    }

    /// Checks if the bound chunked array contains any unsaturated bonds.
    pub fn is_unsaturated(&self) -> bool {
        self.into_iter()
            .any(|bound| bound.is_some_and(Bound::is_unsaturated))
    }

    pub fn is_monounsaturated(&self) -> bool {
        self.unsaturated().count() == 1
    }

    pub fn is_polyunsaturated(&self) -> bool {
        self.unsaturated().count() > 1
    }

    pub fn is_cis(&self) -> bool {
        self.into_iter()
            .all(|bound| bound.is_some_and(Bound::is_cis))
    }

    pub fn is_trans(&self) -> bool {
        self.into_iter()
            .any(|bound| bound.is_some_and(Bound::is_trans))
    }
}
