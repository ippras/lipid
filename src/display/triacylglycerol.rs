use crate::display::{Delta, FattyAcid};
use polars::prelude::*;
use std::{
    fmt::{Display, Formatter, Result, Write, from_fn},
    ops::Index,
};

/// Triacylglycerol
#[derive(Clone, Copy, Debug, Default)]
pub struct Triacylglycerol<T>(pub [T; 3]);

impl<T> Triacylglycerol<T> {
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Triacylglycerol<U> {
        Triacylglycerol(self.0.map(f))
    }
}

impl<T> Triacylglycerol<T> {
    pub fn mono(&self) -> Mono<&Self> {
        Mono(self)
    }

    pub fn positional(&self) -> Positional<&Self> {
        Positional(self)
    }

    pub fn stereo(&self) -> Stereo<&Self> {
        Stereo(self)
    }
}

// impl<T: Display> Display for Triacylglycerol<Option<T>> {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         let mut stereospecific_number = |number| {
//             from_fn(|f| match &self.0[number] {
//                 Some(value) => Display::fmt(value, f),
//                 None => f.write_str("None"),
//             })
//         };
//         write!(
//             f,
//             "{}, {}, {}",
//             stereospecific_number(0),
//             stereospecific_number(1),
//             stereospecific_number(2),
//         )
//     }
// }

impl<T: Display> Display for Triacylglycerol<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}, {}, {}", self.0[0], self.0[1], self.0[2])
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

// impl NonStereospecific<&Triacylglycerol> {
//     pub fn delta(&self) -> NonStereospecific<Triacylglycerol<Option<Delta<FattyAcid>>>> {
//         NonStereospecific(Triacylglycerol(
//             self.0.0.map(|fatty_acid| Some(Delta(fatty_acid?))),
//         ))
//     }
// }

// impl Display for NonStereospecific<&Triacylglycerol> {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         let [sn1, sn2, sn3] = &self.0.0;
//         let t = || match sn1 {
//             Some(fatty_acid) => todo!(),
//             None => todo!(),
//         };
//         // write!(f, "[{sn1}|{sn2}|{sn3}]")
//         f.write_char('[')?;
//         f.write_char(']')
//     }
// }

impl<T: Display> Display for Mono<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if f.alternate() {
            f.write_str("Mono")?;
        }
        write!(f, "({})", self.0)
    }
}

/// Positional
#[derive(Clone, Copy, Debug, Default)]
pub struct Positional<T>(pub T);

impl<T: Display> Display for Positional<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if f.alternate() {
            f.write_str("Positional")?;
        }
        write!(f, "{{{}}}", self.0)
    }
}

/// Stereo
#[derive(Clone, Copy, Debug, Default)]
pub struct Stereo<T>(pub T);

impl<T: Display> Display for Stereo<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if f.alternate() {
            f.write_str("Stereo")?;
        }
        write!(f, "[{}]", self.0)
    }
}
