use super::Triacylglycerol;
use crate::r#const::EM_DASH;
use std::fmt::{Display, Formatter, Result, from_fn};

impl<T: Display> Triacylglycerol<Option<T>> {
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

/// Mono
#[derive(Clone, Copy, Debug, Default)]
pub struct Mono<T>(pub T);

impl<T: Display> Display for Mono<&Triacylglycerol<Option<T>>> {
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
            write!(f, "[{sn1}/3;{sn2}/3;{sn3}/3]")
        }
    }
}

/// Positional
#[derive(Clone, Copy, Debug, Default)]
pub struct Positional<T>(pub T);

impl<T: Display> Display for Positional<&Triacylglycerol<Option<T>>> {
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

impl<T: Display> Display for Stereo<&Triacylglycerol<Option<T>>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let sn1 = option(&self.0[0]);
        let sn2 = option(&self.0[1]);
        let sn3 = option(&self.0[2]);
        if f.alternate() {
            write!(f, "{{1:{sn1} & 2:{sn2} & 3:{sn3}}}")
        } else {
            write!(f, "[{sn1};{sn2};{sn3}]")
        }
    }
}

fn option<T: Display>(option: &Option<T>) -> impl Display {
    from_fn(move |f| match option {
        None => f.write_str(EM_DASH),
        Some(t) => Display::fmt(t, f),
    })
}
