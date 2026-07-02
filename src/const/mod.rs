pub mod fatty_acid;
pub mod triacylglycerol;

pub use self::fatty_acid::{explicit::*, implicit::*};

/// Fatty acid carbon column name
pub const CARBON: &str = "Carbon";
/// End
pub const END: &str = "End";
/// Fatty acid column name
pub const FATTY_ACID: &str = "FattyAcid";
/// Fatty acid bound index column name
pub const INDEX: &str = "Index";
/// Fatty acid indices column name
pub const INDICES: &str = "Indices";
/// Fatty acid bound parity column name
pub const PARITY: &str = "Parity";
/// Range
pub const RANGE: &str = "Range";
/// Start
pub const START: &str = "Start";
/// Fatty acid bound triple column name
pub const TRIPLE: &str = "Triple";
/// Unsaturated
pub const UNSATURATED: &str = "Unsaturated";

/// Triacylglycerol
pub const TRIACYLGLYCEROL: &str = "Triacylglycerol";
/// Stereospecific numbers
pub const STEREOSPECIFIC_NUMBERS: &str = "StereospecificNumbers";
/// Stereospecific numbers 1
pub const STEREOSPECIFIC_NUMBERS1: &str = "StereospecificNumbers1";
/// Stereospecific numbers 2
pub const STEREOSPECIFIC_NUMBERS2: &str = "StereospecificNumbers2";
/// Stereospecific numbers 3
pub const STEREOSPECIFIC_NUMBERS3: &str = "StereospecificNumbers3";
/// Stereospecific numbers (1 and 2) or (2 and 3)
pub const STEREOSPECIFIC_NUMBERS12_23: &str = "StereospecificNumbers12(23)";
/// Stereospecific numbers 1 and 3
pub const STEREOSPECIFIC_NUMBERS13: &str = "StereospecificNumbers13";
/// Stereospecific numbers 1 or 3
pub const STEREOSPECIFIC_NUMBERS1_3: &str = "StereospecificNumbers1(3)";
/// Stereospecific numbers 1 and 2 and 3
pub const STEREOSPECIFIC_NUMBERS123: &str = "StereospecificNumbers123";
/// Label
pub const LABEL: &str = "Label";

// mod wildcard {
//     fatty_acid!(C26U2X);
//     fatty_acid!(C26U3X);
//     fatty_acid!(C26U4X);
//     fatty_acid!(C26U5X);
//     fatty_acid!(C26U6X);
//     fatty_acid!(C28U1X);
//     fatty_acid!(C28U2X);
//     fatty_acid!(C32U1X);
//     fatty_acid!(C32U2X);
//     fatty_acid!(C34U1X);
//     fatty_acid!(C34U2X);
//     fatty_acid!(C36U1X);
//     fatty_acid!(C36U2X);
// }
