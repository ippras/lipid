pub use self::{
    fatty_acid::{CARBON, FATTY_ACID, INDEX, INDICES, PARITY, TRIPLE, explicit::*, implicit::*},
    triacylglycerol::{
        LABEL, STEREOSPECIFIC_NUMBERS, STEREOSPECIFIC_NUMBERS1, STEREOSPECIFIC_NUMBERS1_3,
        STEREOSPECIFIC_NUMBERS2, STEREOSPECIFIC_NUMBERS3, STEREOSPECIFIC_NUMBERS12_23,
        STEREOSPECIFIC_NUMBERS13, STEREOSPECIFIC_NUMBERS123, TRIACYLGLYCEROL,
    },
};

pub mod fatty_acid;
pub mod triacylglycerol;

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
