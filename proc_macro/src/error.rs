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
    #[error(
        "fatty_acid: unsaturated value greater or equal then carbon value carbon_value={carbon_value:?} unsaturated_value={unsaturated_value:?}"
    )]
    UnsaturatedGreaterOrEqualThanCarbon {
        carbon_value: usize,
        unsaturated_value: usize,
    },
    #[error(
        "fatty_acid: the number of indices provided exceeds the allowed unsaturated indices_length={indices_length:?} unsaturated_value={unsaturated_value:?}"
    )]
    IndicesGreaterThanUnsaturated {
        indices_length: usize,
        unsaturated_value: usize,
    },
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SpannedError {
    pub(crate) span: Span,
    pub(crate) error: Error,
}

impl From<SpannedError> for syn::Error {
    fn from(value: SpannedError) -> Self {
        syn::Error::new(value.span, value.error)
    }
}

// ParseCarbonKeyword,
// ParseCarbonValue,
// StartGreaterThanStop,
// UnexpectedEntryKey,
// OmegaOutOfBounds,

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
