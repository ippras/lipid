pub use self::fatty_acid::{explicit::*, implicit::*};

pub mod fatty_acid;
pub mod triacylglycerol;

/// Area
pub const AREA: &str = "Area";
/// Fatty acid carbon column name
pub const CARBON: &str = "Carbon";
/// Fatty acid column name
pub const FATTY_ACID: &str = "FattyAcid";
/// Fatty acid bound index column name
pub const INDEX: &str = "Index";
/// Fatty acid indices column name
pub const INDICES: &str = "Indices";
/// Label
pub const LABEL: &str = "Label";
/// Fatty acid bound parity column name
pub const PARITY: &str = "Parity";
/// Retention time
pub const RETENTION_TIME: &str = "RetentionTime";
/// Stereospecific numbers
pub const STEREOSPECIFIC_NUMBERS: &str = "StereospecificNumbers";
/// Stereospecific numbers 1 or 3
pub const STEREOSPECIFIC_NUMBERS1_3: &str = "StereospecificNumbers1(3)";
/// Stereospecific numbers 1
pub const STEREOSPECIFIC_NUMBERS1: &str = "StereospecificNumbers1";
/// Stereospecific numbers (1 and 2) or (2 and 3)
pub const STEREOSPECIFIC_NUMBERS12_23: &str = "StereospecificNumbers12(23)";
/// Stereospecific numbers 1 and 2 and 3
pub const STEREOSPECIFIC_NUMBERS123: &str = "StereospecificNumbers123";
/// Stereospecific numbers 1 and 3
pub const STEREOSPECIFIC_NUMBERS13: &str = "StereospecificNumbers13";
/// Stereospecific numbers 2
pub const STEREOSPECIFIC_NUMBERS2: &str = "StereospecificNumbers2";
/// Stereospecific numbers 3
pub const STEREOSPECIFIC_NUMBERS3: &str = "StereospecificNumbers3";
/// Total lipids
pub const TOTAL_LIPIDS: &str = "TotalLipids";
/// Triacylglycerol
pub const TRIACYLGLYCEROL: &str = "Triacylglycerol";
/// Fatty acid bound triple column name
pub const TRIPLE: &str = "Triple";
/// Value
pub const VALUE: &str = "Value";
