use super::Identifier;
use polars::prelude::{enum_::EnumChunkedBuilder, *};
use std::{
    borrow::Cow,
    fmt::{self, Debug, Display, Formatter, Write, from_fn},
    num::NonZeroI8,
    sync::LazyLock,
};

/// Formula chunked array.
#[derive(Clone)]
pub struct Formula(pub(crate) StructChunked);

impl Formula {
    pub fn atoms(&self) -> PolarsResult<Vec<((Option<u8>, Option<u8>), Option<u8>)>> {
        let carbon = self.0.field_by_name("C")?;
        let hydrogen = self.0.field_by_name("H")?;
        let oxygen = self.0.field_by_name("O")?;
        Ok(carbon
            .u8()?
            .iter()
            .zip(hydrogen.u8()?.iter())
            .zip(oxygen.u8()?.iter())
            .collect())
    }

    // pub fn display(&self) -> PolarsResult<StringChunked> {
    //     self.0.apply_into_string_amortized(|f, buffer| f)
    // }

    // #[inline]
    // pub fn map<T>(
    //     &self,
    //     f: impl Fn(&Int8Chunked) -> T,
    // ) -> impl Iterator<Item = PolarsResult<Option<T>>> {
    //     self.0.amortized_iter().map(move |item| {
    //         let Some(series) = item else {
    //             return Ok(None);
    //         };
    //         Ok(Some(f(series.as_ref().i8()?)))
    //     })
    // }
}

impl Debug for Formula {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Formula");
        if let Ok(carbon) = self.0.field_by_name("C") {
            debug_struct.field("C", &carbon);
        }
        if let Ok(hydrogen) = self.0.field_by_name("H") {
            debug_struct.field("H", &hydrogen);
        }
        if let Ok(oxygen) = self.0.field_by_name("O") {
            debug_struct.field("O", &oxygen);
        }
        debug_struct.finish()
    }
}
