use super::Identifier;
use crate::prelude::{physical::S, *};
use polars::prelude::{enum_::EnumChunkedBuilder, *};
use std::{
    borrow::Cow,
    fmt::{self, Debug, Display, Formatter, Write, from_fn},
    num::NonZeroI8,
    sync::LazyLock,
};

pub const INDEX: &str = "Index";
pub const PARITY: &str = "Parity";

/// Extension methods for [`Series`]
pub trait SeriesExt {
    // fn fatty_acids(&self) -> &FattyAcidListChunked {
    //     self.try_fatty_acid_list().unwrap()
    // }

    fn fatty_acid(&self) -> FattyAcidChunked {
        self.try_fatty_acid().unwrap()
    }

    fn try_fatty_acid(&self) -> PolarsResult<FattyAcidChunked>;
}

/// Fatty acid chunked array.
#[derive(Clone)]
pub struct FattyAcidChunked {
    pub formula: Formula,
    pub double_bounds: ListChunked,
    pub triple_bounds: ListChunked,
}

impl FattyAcidChunked {
    pub fn formula(&self) -> PolarsResult<Vec<((Option<u8>, Option<u8>), Option<u8>)>> {
        self.formula.atoms()
    }

    pub fn double_bounds(
        &self,
    ) -> impl Iterator<Item = PolarsResult<Option<DoubleBoundChunked>>> + ExactSizeIterator {
        self.double_bounds.amortized_iter().map(|series| {
            let Some(series) = series else {
                return Ok(None);
            };
            let r#struct = series.as_ref().struct_()?;
            let index = r#struct.field_by_name(INDEX)?.i8()?.clone();
            let parity = r#struct.field_by_name(PARITY)?.bool()?.clone();
            Ok(Some(DoubleBoundChunked { index, parity }))
        })
    }

    pub fn triple_bounds(
        &self,
    ) -> impl Iterator<Item = PolarsResult<Option<Int8Chunked>>> + ExactSizeIterator {
        self.double_bounds.amortized_iter().map(|series| {
            let Some(series) = series else {
                return Ok(None);
            };
            let index = series.as_ref().i8()?.clone();
            Ok(Some(index))
        })
    }
}

/// Fatty acid chunked array.
#[derive(Clone)]
pub struct Formula(pub StructChunked);

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
}

#[cfg(feature = "atomic")]
impl Atomic for &Formula {
    type Output = PolarsResult<UInt8Chunked>;

    #[inline]
    fn carbons(self) -> PolarsResult<UInt8Chunked> {
        Ok(self.0.field_by_name("C")?.u8()?.clone())
    }

    #[inline]
    fn hydrogens(self) -> PolarsResult<UInt8Chunked> {
        Ok(self.0.field_by_name("H")?.u8()?.clone())
    }
}

/// Double bound chunked array.
#[derive(Clone, Debug)]
pub struct DoubleBoundChunked {
    pub index: Int8Chunked,
    pub parity: BooleanChunked,
}
