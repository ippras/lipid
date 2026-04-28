use super::FattyAcidTrait;
use std::num::NonZeroI8;

/// Fatty acid mask by double bounds
#[allow(clippy::wrong_self_convention)]
pub trait FattyAcidMaskByDoubleBounds: FattyAcidTrait {
    /// Is dienoic
    fn is_dienoic(self) -> Self::Expr;

    /// Is hexaenoic
    fn is_hexaenoic(self) -> Self::Expr;

    /// Is monoenoic
    fn is_monoenoic(self) -> Self::Expr;

    /// Is pentaenoic
    fn is_pentaenoic(self) -> Self::Expr;

    /// Is tetraenoic
    fn is_tetraenoic(self) -> Self::Expr;

    /// Is trienoic
    fn is_trienoic(self) -> Self::Expr;
}

/// Fatty acid mask by parity
#[allow(clippy::wrong_self_convention)]
pub trait FattyAcidMaskByParity: FattyAcidTrait {
    /// Is cis
    fn is_cis(self) -> Self::Expr;

    /// Is trans
    fn is_trans(self) -> Self::Expr;
}

/// Fatty acid mask by saturation
#[allow(clippy::wrong_self_convention)]
pub trait FattyAcidMaskByBounds: FattyAcidTrait {
    /// Is conjugated
    ///
    /// `strict`
    /// * `true` - only double bounds,
    /// * `false` - double and triple bounds.
    fn is_conjugated(self, strict: bool) -> Self::Expr;

    /// Is monounsaturated
    fn is_monounsaturated(self) -> Self::Expr;

    /// Is polyunsaturated
    fn is_polyunsaturated(self) -> Self::Expr;

    /// Is saturated
    fn is_saturated(self) -> Self::Expr;

    /// Is unsaturated
    fn is_unsaturated(self, offset: Option<NonZeroI8>) -> Self::Expr;
}
