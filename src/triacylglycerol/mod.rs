use serde::{Deserialize, Serialize};

/// Stereospecificity
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Stereospecificity {
    Positional,
    Stereo,
}

pub mod polars;
pub mod thermodynamic;
