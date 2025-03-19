use serde::{Deserialize, Serialize};

/// Triacylglycerol
#[derive(Clone, Debug, Hash)]
pub struct Triacylglycerol<T>([Option<T>; 3]);

impl<T> Triacylglycerol<T> {
    /// Create a new triacylglycerol
    pub fn new() -> Self {
        Self([None, None, None])
    }

    /// The first stereospecific number
    pub fn stereospecific_number1(&self) -> Option<&T> {
        self.0[0].as_ref()
    }

    /// The second stereospecific number
    pub fn stereospecific_number2(&self) -> Option<&T> {
        self.0[1].as_ref()
    }

    /// The third stereospecific number
    pub fn stereospecific_number3(&self) -> Option<&T> {
        self.0[2].as_ref()
    }
}

/// Stereospecificity
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Stereospecificity {
    Positional,
    Stereo,
}

pub mod thermodynamic;
