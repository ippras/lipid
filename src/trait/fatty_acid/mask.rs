use super::FattyAcidTrait;
use std::num::NonZeroI8;

/// Fatty acid mask by saturation
#[allow(clippy::wrong_self_convention)]
pub trait FattyAcidMaskByBounds: FattyAcidTrait {
    /// Is conjugated
    ///
    /// `strict`
    /// * `true` - only double bounds,
    /// * `false` - double and triple bounds.
    fn is_conjugated(self, strict: bool) -> Self::Output;

    /// Is monounsaturated
    fn is_monounsaturated(self) -> Self::Output;

    /// Is polyunsaturated
    fn is_polyunsaturated(self) -> Self::Output;

    /// Is saturated
    fn is_saturated(self) -> Self::Output;

    /// Is unsaturated
    fn is_unsaturated(self, offset: Option<NonZeroI8>) -> Self::Output;
}

/// Fatty acid mask by double bounds
#[allow(clippy::wrong_self_convention)]
pub trait FattyAcidMaskByDoubleBounds: FattyAcidTrait {
    /// Is dienoic
    fn is_dienoic(self) -> Self::Output;

    /// Is hexaenoic
    fn is_hexaenoic(self) -> Self::Output;

    /// Is monoenoic
    fn is_monoenoic(self) -> Self::Output;

    /// Is pentaenoic
    fn is_pentaenoic(self) -> Self::Output;

    /// Is tetraenoic
    fn is_tetraenoic(self) -> Self::Output;

    /// Is trienoic
    fn is_trienoic(self) -> Self::Output;
}

/// Fatty acid mask by parity
#[allow(clippy::wrong_self_convention)]
pub trait FattyAcidMaskByParity: FattyAcidTrait {
    /// Is cis
    fn is_cis(self) -> Self::Output;

    /// Is trans
    fn is_trans(self) -> Self::Output;
}
