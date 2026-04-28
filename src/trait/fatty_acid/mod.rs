pub use self::{
    filter::FattyAcidFilter,
    mask::{
        FattyAcidMask, FattyAcidMaskByDoubleBounds, FattyAcidMaskByName, FattyAcidMaskByParity,
        FattyAcidMaskBySaturation,
    },
};

/// Fatty acid trait
pub trait FattyAcidTrait: Sized {
    type Output;
}

mod filter;
mod mask;
