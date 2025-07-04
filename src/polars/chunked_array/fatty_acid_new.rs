use super::Identifier;
use crate::prelude::{physical::S, *};
use polars::prelude::{enum_::EnumChunkedBuilder, *};
use std::{
    borrow::Cow,
    fmt::{self, Debug, Display, Formatter, Write, from_fn},
    num::NonZeroI8,
    sync::LazyLock,
};

pub const DOUBLE_BOUNDS: &str = "DoubleBounds";
pub const FORMULA: &str = "Formula";
pub const INDEX: &str = "Index";
pub const PARITY: &str = "Parity";
pub const TRIPLE_BOUNDS: &str = "TripleBounds";

/// The fatty acid data type.
pub const FATTY_ACID_DATA_TYPE: LazyLock<DataType> = LazyLock::new(|| {
    DataType::Struct(vec![
        Field::new(
            PlSmallStr::from_static(FORMULA),
            DataType::Struct(vec![
                Field::new(PlSmallStr::from_static("C"), DataType::UInt8),
                Field::new(PlSmallStr::from_static("H"), DataType::UInt8),
                Field::new(PlSmallStr::from_static("O"), DataType::UInt8),
            ]),
        ),
        Field::new(
            PlSmallStr::from_static(DOUBLE_BOUNDS),
            DataType::List(Box::new(DataType::Struct(vec![
                Field::new(PlSmallStr::from_static(INDEX), DataType::Int8),
                Field::new(PlSmallStr::from_static(PARITY), DataType::Boolean),
            ]))),
        ),
        Field::new(
            PlSmallStr::from_static(TRIPLE_BOUNDS),
            DataType::List(Box::new(DataType::Int8)),
        ),
    ])
});

/// Extension methods for [`Series`]
pub trait SeriesExt {
    fn double_bound(&self) -> DoubleBoundChunked {
        self.try_double_bound().unwrap()
    }

    fn fatty_acid(&self) -> FattyAcidChunked {
        self.NEW_try_fatty_acid().unwrap()
    }

    fn try_double_bound(&self) -> PolarsResult<DoubleBoundChunked>;

    fn NEW_try_fatty_acid(&self) -> PolarsResult<FattyAcidChunked>;
}

impl SeriesExt for Series {
    fn try_double_bound(&self) -> PolarsResult<DoubleBoundChunked> {
        let r#struct = self.struct_()?;
        let index = r#struct.field_by_name(INDEX)?.i8()?.clone();
        let parity = r#struct.field_by_name(PARITY)?.bool()?.clone();
        Ok(DoubleBoundChunked { index, parity })
    }

    fn NEW_try_fatty_acid(&self) -> PolarsResult<FattyAcidChunked> {
        let r#struct = self.struct_()?;
        let formula = r#struct.field_by_name(FORMULA)?.struct_()?.clone();
        let double_bounds = r#struct.field_by_name(DOUBLE_BOUNDS)?.list()?.clone();
        let triple_bounds = r#struct.field_by_name(TRIPLE_BOUNDS)?.list()?.clone();
        Ok(FattyAcidChunked {
            formula: Formula(formula),
            double_bounds,
            triple_bounds,
        })
    }
}

/// Fatty acid chunked array.
#[derive(Clone, Debug)]
pub struct FattyAcidChunked {
    pub formula: Formula,
    pub double_bounds: ListChunked,
    pub triple_bounds: ListChunked,
}

impl FattyAcidChunked {
    pub fn iter(
        &self,
    ) -> PolarsResult<
        impl Iterator<
            Item = (
                (
                    ((Option<u8>, Option<u8>), Option<u8>),
                    PolarsResult<Option<DoubleBoundChunked>>,
                ),
                PolarsResult<Option<TripleBoundChunked>>,
            ),
        >,
    > {
        Ok(self
            .formula
            .atoms()?
            .into_iter()
            .zip(self.double_bounds())
            .zip(self.triple_bounds()))
    }

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
    ) -> impl Iterator<Item = PolarsResult<Option<TripleBoundChunked>>> + ExactSizeIterator {
        self.triple_bounds.amortized_iter().map(|series| {
            let Some(series) = series else {
                return Ok(None);
            };
            let index = series.as_ref().i8()?.clone();
            Ok(Some(TripleBoundChunked { index }))
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

/// Double bound chunked array.
#[derive(Clone, Debug)]
pub struct DoubleBoundChunked {
    pub index: Int8Chunked,
    pub parity: BooleanChunked,
}

impl DoubleBoundChunked {
    #[inline]
    pub fn is_saturated(&self) -> bool {
        self.parity.len() == 0
    }

    #[inline]
    pub fn is_unsaturated(&self, offset: Option<NonZeroI8>) -> bool {
        !self.is_saturated()
    }

    // #[inline]
    // pub fn is_monounsaturated(&self) -> bool {
    //     self.double_bounds.len() + self.triple_bounds.len() == 1
    // }

    // #[inline]
    // pub fn is_polyunsaturated(&self) -> bool {
    //     self.double_bounds.len() + self.triple_bounds.len() > 1
    // }

    // #[inline]
    // pub fn is_cis(&self) -> PolarsResult<bool> {
    //     Ok(self
    //         .double_bounds
    //         .explode(true)?
    //         .struct_()?
    //         .field_by_name(PARITY)?
    //         .bool()?
    //         .any())
    // }

    // #[inline]
    // pub fn is_trans(&self) -> PolarsResult<bool> {
    //     Ok(!self
    //         .double_bounds
    //         .explode(true)?
    //         .struct_()?
    //         .field_by_name(PARITY)?
    //         .bool()?
    //         .all())
    // }
}

/// Triple bound chunked array.
#[derive(Clone, Debug)]
pub struct TripleBoundChunked {
    pub index: Int8Chunked,
}
