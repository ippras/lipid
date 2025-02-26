use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// Fatty acid
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct FattyAcid {
    pub bounds: Vec<Option<Bound>>,
}

impl FattyAcid {
    pub fn new(carbons: u8) -> Self {
        Self {
            bounds: vec![None; carbons as usize - 1],
        }
    }
}
