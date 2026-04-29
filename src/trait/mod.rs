use crate::prelude::*;

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

// /// Extension methods for [`Mask`].
// pub trait MaskExt: IdentifierMask {
//     fn try_unsaturated(self, index: Option<NonZeroI8>) -> Self::Output;

//     fn is_delta_unsaturated(self, index: NonZeroU8) -> Self::Output;

//     fn is_omega_unsaturated(self, index: NonZeroU8) -> Self::Output;
// }

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

pub mod fatty_acid;
