use super::FattyAcidTrait;
use std::num::NonZeroI8;

/// Fatty acid mask
pub trait FattyAcidMask: FattyAcidTrait {
    /// Is conjugated
    ///
    /// `strict`
    /// * `true` - only double bounds,
    /// * `false` - double and triple bounds.
    fn is_conjugated(self, strict: bool) -> Self::Output;
}

/// Fatty acid mask by double bounds
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

/// Fatty acid mask by name
pub trait FattyAcidMaskByName: FattyAcidTrait {
    /// α-Linolenic acid
    fn is_alpha_linolenic(self) -> Self::Output;

    /// Butyric acid
    fn is_butyric(self) -> Self::Output;

    /// Docosahexaenoic acid (DHA)
    fn is_docosahexaenoic(self) -> Self::Output;

    /// Eicosapentaenoic acid (EPA)
    fn is_eicosapentaenoic(self) -> Self::Output;

    /// Linoleic acid
    fn is_linoleic(self) -> Self::Output;

    /// Oleic acid
    fn is_oleic(self) -> Self::Output;
}

/// Fatty acid mask by parity
pub trait FattyAcidMaskByParity: FattyAcidTrait {
    /// Is cis
    fn is_cis(self) -> Self::Output;

    /// Is trans
    fn is_trans(self) -> Self::Output;
}

/// Fatty acid mask by saturation
pub trait FattyAcidMaskBySaturation: FattyAcidTrait {
    /// Is monounsaturated
    fn is_monounsaturated(self) -> Self::Output;

    /// Is polyunsaturated
    fn is_polyunsaturated(self) -> Self::Output;

    /// Is saturated
    fn is_saturated(self) -> Self::Output;

    /// Is unsaturated
    fn is_unsaturated(self, offset: Option<NonZeroI8>) -> Self::Output;
}
