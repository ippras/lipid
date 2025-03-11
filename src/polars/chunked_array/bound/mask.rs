use crate::prelude::*;
use std::{convert::identity, num::NonZeroI8};

impl BoundChunked {
    /// Checks if the bounds chunked array contains only saturated bonds.
    ///
    /// # Returns
    ///
    /// [`true`] if all bounds are saturated, [`false`] otherwise.
    pub fn is_saturated(&self) -> bool {
        self.into_iter()
            .all(|bound| bound.is_some_and(Bound::is_saturated))
    }

    /// Checks if the bound chunked array contains any unsaturated bonds.
    ///
    /// # Returns
    ///
    /// [`true`] if any bound is unsaturated, [`false`] otherwise.
    pub fn is_unsaturated(&self, n: Option<NonZeroI8>) -> bool {
        match n {
            Some(n) => {
                let abs = n.unsigned_abs().into();
                if n.is_positive() {
                    self.unsaturated().next() == Some(abs - 1)
                } else {
                    self.unsaturated().last() == Some(self.len().saturating_sub(abs))
                }
            }
            None => self
                .into_iter()
                .any(|bound| bound.is_some_and(Bound::is_unsaturated)),
        }
    }

    /// Checks if the bound chunked array contains exactly one unsaturated bond.
    ///
    /// # Returns
    ///
    /// [`true`] if there is exactly one unsaturated bond, [`false`] otherwise.
    pub fn is_monounsaturated(&self) -> bool {
        self.unsaturated().count() == 1
    }

    /// Checks if the bound chunked array contains more than one unsaturated bond.
    ///
    /// # Returns
    ///
    /// [`true`] if there are more than one unsaturated bonds, [`false`] otherwise.
    pub fn is_polyunsaturated(&self) -> bool {
        self.unsaturated().count() > 1
    }

    /// Checks if the bound chunked array contains unsaturated cis-only bonds.
    ///
    /// # Returns
    ///
    /// [`true`] if all unsaturated bounds are cis, [`false`] otherwise.
    pub fn is_cis(&self) -> bool {
        self.into_iter()
            .fold(None, |is_cis, bound| match bound {
                Some(bound) if bound.is_cis() => Some(is_cis.unwrap_or(true)),
                Some(bound) if bound.is_trans() => Some(false),
                _ => is_cis,
            })
            .is_some_and(identity)
    }

    /// Checks if the bound chunked array contains any trans bonds.
    ///
    /// # Returns
    ///
    /// [`true`] if any bound is trans, [`false`] otherwise.
    pub fn is_trans(&self) -> bool {
        self.into_iter()
            .any(|bound| bound.is_some_and(Bound::is_trans))
    }
}
