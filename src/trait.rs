use crate::prelude::*;
use std::num::{NonZeroI8, NonZeroU8};

pub trait Atomic {
    type Output;

    /// Returns the number of carbon atoms in the principle chain.
    fn carbon(self) -> Self::Output;

    /// Returns the number of hydrogen atoms.
    fn hydrogen(self) -> Self::Output;

    /// Returns the number of oxygen atoms.
    fn oxygen(self) -> Self::Output;
}

pub trait EquivalentCarbonNumber {
    type Output;

    /// Returns the number of carbon atoms in the principle chain.
    fn equivalent_carbon_number(self) -> Self::Output;
}

pub trait EquivalentChainLength {
    type Output;

    fn equivalent_chain_length(
        self,
        retention_time: Self::Output,
        logarithmic: bool,
    ) -> Self::Output;

    fn fractional_chain_length(
        self,
        retention_time: Self::Output,
        logarithmic: bool,
    ) -> Self::Output;
}

/// Mask
pub trait IdentifierMask {
    type Output;

    /// Checks if the fatty acid contains only saturated bonds.
    fn is_saturated(self) -> Self::Output;

    /// Checks if the fatty acid contains any unsaturated bonds.
    fn is_unsaturated(self) -> Self::Output;

    /// Checks if the fatty acid contains exactly one unsaturated bond.
    fn is_monounsaturated(self) -> Self::Output;

    /// Checks if the fatty acid contains more than one unsaturated bond.
    fn is_polyunsaturated(self) -> Self::Output;

    /// Checks if the fatty acid contains unsaturated cis-only bonds.
    fn is_cis(self) -> Self::Output;

    /// Checks if the fatty acid contains any trans bonds.
    fn is_trans(self) -> Self::Output;
}

/// Extension methods for [`Mask`].
pub trait MaskExt: IdentifierMask {
    fn try_unsaturated(self, index: Option<NonZeroI8>) -> Self::Output;

    fn is_delta_unsaturated(self, index: NonZeroU8) -> Self::Output;

    fn is_omega_unsaturated(self, index: NonZeroU8) -> Self::Output;
}

pub trait RelativeAtomicMass {
    type Output;

    /// Returns the number of carbon atoms in the principle chain.
    fn relative_atomic_mass(self, adduct: Option<Self::Output>) -> Self::Output;
}

pub trait Kind: RelativeAtomicMass {
    fn rco(&self) -> Rco<&Self> {
        Rco(self)
    }

    fn rcoo(&self) -> Rcoo<&Self> {
        Rcoo(self)
    }

    fn rcooh(&self) -> Rcooh<&Self> {
        Rcooh(self)
    }

    fn rcooch3(&self) -> Rcooch3<&Self> {
        Rcooch3(self)
    }
}

// impl IndexedIdentifierChunked {
//     pub fn rco(&self) -> Rco<&Self> {
//         Rco(self)
//     }

//     pub fn rcoo(&self) -> Rcoo<&Self> {
//         Rcoo(self)
//     }

//     pub fn rcooh(&self) -> Rcooh<&Self> {
//         Rcooh(self)
//     }

//     pub fn rcooch3(&self) -> Rcooch3<&Self> {
//         Rcooch3(self)
//     }
// }

/// Fatty acid mask
pub trait FattyAcidMask: Sized {
    type Output;

    /// Is cis
    fn is_cis(self) -> Self::Output;

    /// Is trans
    fn is_trans(self) -> Self::Output;

    /// Is conjugated
    ///
    /// `strict`
    /// * `true` - only double bounds,
    /// * `false` - double and triple bounds.
    fn is_conjugated(self, strict: bool) -> Self::Output;

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

    /// Is monounsaturated
    fn is_monounsaturated(self) -> Self::Output;

    /// Is polyunsaturated
    fn is_polyunsaturated(self) -> Self::Output;

    /// Is saturated
    fn is_saturated(self) -> Self::Output;

    /// Is unsaturated
    fn is_unsaturated(self, offset: Option<NonZeroI8>) -> Self::Output;
}

/// Fatty acid filter
pub trait FattyAcidFilter: FattyAcidMask {
    fn dienoics(self, expr: Self::Output) -> Self::Output;
    fn hexaenoics(self, expr: Self::Output) -> Self::Output;
    fn monoenoics(self, expr: Self::Output) -> Self::Output;
    fn pentaenoics(self, expr: Self::Output) -> Self::Output;
    fn tetraenoics(self, expr: Self::Output) -> Self::Output;
    fn trienoic(self, expr: Self::Output) -> Self::Output;
}
