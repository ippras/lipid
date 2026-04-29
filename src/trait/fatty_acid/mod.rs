pub use self::{
    mask::{FattyAcidMaskByBounds, FattyAcidMaskByDoubleBounds, FattyAcidMaskByParity},
    sum::{FattyAcidSumByBounds, FattyAcidSumByDoubleBounds},
};

/// Fatty acid trait
pub trait FattyAcidTrait: Sized {
    type Output;
}

mod mask;
mod sum;
