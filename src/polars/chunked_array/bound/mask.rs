use crate::prelude::*;
use polars::prelude::*;
use std::num::{NonZero, NonZeroI8, NonZeroU8};

impl Mask for &BoundChunked {
    type Output = PolarsResult<Option<bool>>;

    /// # Returns
    ///
    /// The output is unknown (`None`) if the array contains any null values and
    /// no `false` values.
    fn is_saturated(self) -> PolarsResult<Option<bool>> {
        Ok(self.equal(S)?.all_kleene())
    }

    /// Checks if the bound chunked array contains any unsaturated bonds.
    ///
    /// # Returns
    ///
    /// The output is unknown (`None`) if the array contains any null values and
    /// no `true` values.
    fn is_unsaturated(self) -> PolarsResult<Option<bool>> {
        Ok(self.not_equal(S)?.any_kleene())
    }

    /// Checks if the bound chunked array contains exactly one unsaturated bond.
    ///
    /// # Returns
    ///
    /// [`true`] if there is exactly one unsaturated bond, [`false`] otherwise.
    fn is_monounsaturated(self) -> PolarsResult<Option<bool>> {
        let mask = self.not_equal(S)?;
        if mask.has_nulls() {
            Ok(None)
        } else {
            Ok(Some(mask.num_trues() == 1))
        }
    }

    /// Checks if the bound chunked array contains more than one unsaturated bond.
    ///
    /// # Returns
    ///
    /// [`true`] if there are more than one unsaturated bonds, [`false`] otherwise.
    fn is_polyunsaturated(self) -> PolarsResult<Option<bool>> {
        let mask = self.not_equal(S)?;
        let trues = mask.num_trues();
        if trues > 1 {
            Ok(Some(true))
        } else if trues + mask.null_count() > 1 {
            Ok(None)
        } else {
            Ok(Some(false))
        }
    }

    /// Checks if the bound chunked array contains unsaturated cis-only bonds.
    ///
    /// # Returns
    ///
    /// [`true`] if all unsaturated bounds are cis, [`false`] otherwise.
    fn is_cis(self) -> PolarsResult<Option<bool>> {
        let mut is_cis = Some(false);
        for bound in self.try_iter() {
            let bound = bound?;
            is_cis = match bound {
                Some(Bound::Saturated) => is_cis,
                Some(Bound::Unsaturated(Unsaturated {
                    isomerism: Some(Isomerism::Cis),
                    ..
                })) => Some(true),
                Some(Bound::Unsaturated(Unsaturated {
                    isomerism: Some(Isomerism::Trans),
                    ..
                })) => return Ok(Some(false)),
                _ => None,
            };
        }
        Ok(is_cis)
    }

    /// Checks if the bound chunked array contains any trans bonds.
    ///
    /// # Returns
    ///
    /// [`true`] if any bound is trans, [`false`] otherwise.
    fn is_trans(self) -> PolarsResult<Option<bool>> {
        let mut is_trans = Some(false);
        for bound in self.try_iter() {
            let bound = bound?;
            is_trans = match bound {
                Some(Bound::Saturated) => is_trans,
                Some(Bound::Unsaturated(Unsaturated {
                    isomerism: Some(Isomerism::Cis),
                    ..
                })) => is_trans,
                Some(Bound::Unsaturated(Unsaturated {
                    isomerism: Some(Isomerism::Trans),
                    ..
                })) => return Ok(Some(true)),
                _ => None,
            };
        }
        Ok(is_trans)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Unsized {
    saturated: usize,
    unsaturated: usize,
    any: usize,
}
