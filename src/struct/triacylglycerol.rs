use polars::prelude::*;
use std::{
    fmt::{Display, Formatter, Result, from_fn},
    ops::Index,
};

use crate::r#const::EM_DASH;

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

impl<T: Display> Display for Triacylglycerol<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}; {}; {}", self.0[0], self.0[1], self.0[2])
    }
}

impl<T> Index<usize> for Triacylglycerol<T> {
    type Output = <[T] as Index<usize>>::Output;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// Mono
#[derive(Clone, Copy, Debug, Default)]
pub struct Mono<T>(pub T);

impl<T: Display> Display for Mono<Triacylglycerol<Option<T>>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let sn1 = option(&self.0[0]);
        let sn2 = option(&self.0[1]);
        let sn3 = option(&self.0[2]);
        if f.alternate() {
            write!(
                f,
                "{{1:{sn1} & 2:{sn2} & 3:{sn3} | 1:{sn1} & 2:{sn3} & 3:{sn2} | 1:{sn2} & 2:{sn1} & 3:{sn3} | 1:{sn2} & 2:{sn3} & 3:{sn1} | 1:{sn3} & 2:{sn1} & 3:{sn2} | 1:{sn3} & 2:{sn2} & 3:{sn1}}}"
            )
        } else {
            write!(f, "[{sn1};{sn2};{sn3}]")
        }
    }
}

/// Positional
#[derive(Clone, Copy, Debug, Default)]
pub struct Positional<T>(pub T);

impl<T: Display> Display for Positional<Triacylglycerol<Option<T>>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let sn1 = option(&self.0[0]);
        let sn2 = option(&self.0[1]);
        let sn3 = option(&self.0[2]);
        if f.alternate() {
            write!(
                f,
                "{{1:{sn1} & 2:{sn2} & 3:{sn3} | 1:{sn3} & 2:{sn2} & 3:{sn1}}}"
            )
        } else {
            write!(f, "[{sn1}/2;{sn2};{sn3}/2]")
        }
    }
}

/// Stereo
#[derive(Clone, Copy, Debug, Default)]
pub struct Stereo<T>(pub T);

impl<T: Display> Display for Stereo<Triacylglycerol<Option<T>>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let sn1 = option(&self.0[0]);
        let sn2 = option(&self.0[1]);
        let sn3 = option(&self.0[2]);
        if f.alternate() {
            write!(f, "{{1:{sn1} & 2:{sn2} & 3:{sn3}}}")
        } else {
            write!(f, "[{sn1}/3;{sn2}/3;{sn3}/3]")
        }
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

fn option<T: Display>(option: &Option<T>) -> impl Display {
    from_fn(move |f| match option {
        None => f.write_str(EM_DASH),
        Some(t) => Display::fmt(t, f),
    })
}
