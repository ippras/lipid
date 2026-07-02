macro_rules! error {
    ($error:ident : $span:expr) => {
        Err(syn::Error::new($span.span(), $error.message()))
    };
}

pub(crate) use error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Error {
    StartGreaterThanStop,
    UnexpectedEntryKey,
    OmegaOutOfBounds,
    IndicesLengthGreaterThanRange,
}

impl Error {
    pub(crate) fn message(&self) -> &'static str {
        match self {
            Self::StartGreaterThanStop => "FattyAcid: `start` cannot be greater than `end`",
            Self::UnexpectedEntryKey => {
                "FattyAcid: unexpected entry key, expected one of: C, T, O, A, U"
            }
            Self::OmegaOutOfBounds => {
                "FattyAcid: offset is out of bounds (absolute value is greater than carbons end)"
            }
            Self::IndicesLengthGreaterThanRange => {
                "FattyAcid: the number of indices provided exceeds the minimum allowed unsaturations (`start` value in unsaturated range)"
            }
        }
    }
}
