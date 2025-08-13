pub use self::{
    fatty_acid::{Delta, FattyAcid, Id, Unsaturated},
    triacylglycerol::{Mono, Positional, Stereo, Triacylglycerol},
};

/// Display common options
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Options {
    pub bounds: Elision,
    pub isomerism: Elision,
}

/// Elision
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Elision {
    Explicit,
    #[default]
    Implicit,
}

mod fatty_acid;
mod triacylglycerol;
