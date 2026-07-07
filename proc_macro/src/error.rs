macro_rules! error {
    ($error:expr) => {
        Err(syn::Error::new($span, $error.into()))
    };
}

pub(crate) use error;

use proc_macro2::Span;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Error)]
pub(crate) enum Error {
    // ParseCarbonKeyword,
    // ParseCarbonValue,
    // StartGreaterThanStop,
    // UnexpectedEntryKey,
    // OmegaOutOfBounds,
    #[error(
        "FattyAcid: unsaturated counts greater or equal then carbons count carbons_count={carbons_count:?} unsaturated_count={unsaturated_count:?}"
    )]
    UnsaturatedCountGreaterOrEqualThanCarbonsCount {
        span: Span,
        carbons_count: usize,
        unsaturated_count: usize,
    },
    #[error(
        "FattyAcid: the number of indices provided exceeds the allowed unsaturated indices_length={indices_length:?} unsaturated_count={unsaturated_count:?}"
    )]
    IndicesLengthGreaterThanUnsaturatedCount {
        span: Span,
        indices_length: usize,
        unsaturated_count: usize,
    },
}

// impl Error {
//     pub(crate) fn message(&self) -> &'static str {
//         match self {
//             Self::ParseCarbonKeyword => "FattyAcid: parse carbon keyword (`C`)",
//             Self::ParseCarbonValue => "FattyAcid: parse carbon value",
//             Self::StartGreaterThanStop => "FattyAcid: `start` cannot be greater than `end`",
//             Self::UnexpectedEntryKey => {
//                 "FattyAcid: unexpected entry key, expected one of: C, T, O, A, U"
//             }
//             Self::OmegaOutOfBounds => {
//                 "FattyAcid: offset is out of bounds (absolute value is greater than carbons end)"
//             }
//             Self::IndicesLengthGreaterThanUnsaturated => {
//                 "FattyAcid: the number of indices provided exceeds the minimum allowed unsaturations (`start` value in unsaturated range)"
//             }
//         }
//     }
// }

impl From<Error> for syn::Error {
    fn from(value: Error) -> Self {
        syn::Error::new(Span::call_site(), "message")
    }
}
