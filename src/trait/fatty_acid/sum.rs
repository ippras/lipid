use super::FattyAcidTrait;
use std::num::NonZeroI8;

/// Fatty acid sum by bounds
pub trait FattyAcidSumByBounds: FattyAcidTrait {
    /// Conjugated fatty acids (CFA).
    ///
    /// Conjugated fatty acids have two or more conjugated double bonds.
    fn sum_conjugated(self, expr: Self::Output) -> Self::Output;

    /// Monounsaturated fatty acids (MUFA).
    ///
    /// All unsaturated fatty acids having only one unsaturated bond.
    fn sum_monounsaturated(self, expr: Self::Output) -> Self::Output;

    /// Polyunsaturated fatty acids (PUFA).
    ///
    /// All unsaturated fatty acids having more than one unsaturated bond.
    fn sum_polyunsaturated(self, expr: Self::Output) -> Self::Output;

    /// Saturated fatty acids (SFA).
    ///
    /// All saturated fatty acids
    fn sum_saturated(self, expr: Self::Output) -> Self::Output;

    /// Trans fatty acids (TFA).
    ///
    /// All trans fatty acids.
    fn sum_trans(self, expr: Self::Output) -> Self::Output;

    /// Unsaturated fatty acids (UFA).
    ///
    /// All unsaturated fatty acids
    fn sum_unsaturated(self, expr: Self::Output, offset: Option<NonZeroI8>) -> Self::Output;
}

/// Fatty acid sum by double bounds
pub trait FattyAcidSumByDoubleBounds: FattyAcidTrait {
    fn sum_dienoics(self, expr: Self::Output) -> Self::Output;

    fn sum_hexaenoics(self, expr: Self::Output) -> Self::Output;

    fn sum_monoenoics(self, expr: Self::Output) -> Self::Output;

    fn sum_pentaenoics(self, expr: Self::Output) -> Self::Output;

    fn sum_tetraenoics(self, expr: Self::Output) -> Self::Output;

    fn sum_trienoic(self, expr: Self::Output) -> Self::Output;
}
