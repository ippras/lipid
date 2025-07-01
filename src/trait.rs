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

pub trait Mass {
    type Output;

    /// Returns the number of carbon atoms in the principle chain.
    fn mass(self, adduct: Option<Self::Output>) -> Self::Output;
}

pub trait Kind: Mass {
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
