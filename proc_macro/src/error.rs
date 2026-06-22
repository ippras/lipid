macro_rules! error {
    ($error:ident : $span:expr) => {
        Err(syn::Error::new($span.span(), $error.message()))
    };
}

pub(crate) use error;

#[derive(Debug, Clone)]
pub(crate) enum Error {
    StartGreaterThanStop,
    UnexpectedEntryKey,
    OmegaOutOfBounds,
    IndicesLengthGreaterThanRange,
}

impl Error {
    pub(crate) fn message(&self) -> &'static str {
        match self {
            Self::StartGreaterThanStop => "FattyAcid: `start` cannot be greater than `stop`",
            Self::UnexpectedEntryKey => {
                "FattyAcid: unexpected entry key, expected one of: C, T, O, A, U"
            }
            Self::OmegaOutOfBounds => {
                "FattyAcid: omega offset is out of bounds (absolute value is greater than C stop)"
            }
            Self::IndicesLengthGreaterThanRange => {
                "FattyAcid: the number of indices provided exceeds the maximum allowed unsaturations (`end` value in U range)"
            }
        }
    }
}
