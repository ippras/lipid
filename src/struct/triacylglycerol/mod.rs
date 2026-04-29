use polars::prelude::*;
use std::ops::Index;

/// Triacylglycerol
#[derive(Clone, Copy, Debug, Default)]
pub struct Triacylglycerol<T>(pub [T; 3]);

impl<T> Triacylglycerol<T> {
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Triacylglycerol<U> {
        Triacylglycerol(self.0.map(f))
    }

    pub fn try_map<U>(self, f: impl Fn(T) -> PolarsResult<U>) -> PolarsResult<Triacylglycerol<U>> {
        Ok(Triacylglycerol(self.0.try_map(f)?))
    }
}

impl<T> Index<usize> for Triacylglycerol<T> {
    type Output = <[T] as Index<usize>>::Output;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// /// Stereospecificity
// #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub enum Stereospecificity {
//     Mono,
//     Positional,
//     Stereo,
// }

/// Stereospecificity
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Stereospecificity {
    Stereo,
    Positional,
}

pub mod display;
