use std::{
    fmt::{Display, Formatter, Result},
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

impl<T: Display> Display for Mono<Triacylglycerol<T>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if f.alternate() {
            write!(
                f,
                "{{1:{0} & 2:{1} & 3:{2} | 1:{0} & 2:{2} & 3:{1} | 1:{1} & 2:{0} & 3:{2} | 1:{1} & 2:{2} & 3:{0} | 1:{2} & 2:{0} & 3:{1} | 1:{2} & 2:{1} & 3:{0}}}",
                self.0[0], self.0[1], self.0[2]
            )
        } else {
            write!(f, "[{};{};{}]", self.0[0], self.0[1], self.0[2])
        }
    }
}

/// Positional
#[derive(Clone, Copy, Debug, Default)]
pub struct Positional<T>(pub T);

impl<T: Display> Display for Positional<Triacylglycerol<T>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if f.alternate() {
            write!(
                f,
                "{{1:{0} & 2:{1} & 3:{2} | 1:{2} & 2:{1} & 3:{0}}}",
                self.0[0], self.0[1], self.0[2]
            )
        } else {
            write!(f, "[{}/2;{};{}/2]", self.0[0], self.0[1], self.0[2])
        }
    }
}

/// Stereo
#[derive(Clone, Copy, Debug, Default)]
pub struct Stereo<T>(pub T);

impl<T: Display> Display for Stereo<Triacylglycerol<T>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if f.alternate() {
            write!(
                f,
                "{{1:{0} & 2:{1} & 3:{2}}}",
                self.0[0], self.0[1], self.0[2]
            )
        } else {
            write!(f, "[{}/3;{}/3;{}/3]", self.0[0], self.0[1], self.0[2])
        }
    }
}
