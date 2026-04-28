use crate::prelude::*;
use polars::prelude::*;

/// Unsaturated Chunked
#[repr(transparent)]
pub struct UnsaturatedChunked(StructChunked);

impl UnsaturatedChunked {
    pub(super) fn new(r#struct: &StructChunked) -> &Self {
        // [safe](https://doc.rust-lang.org/reference/type-layout.html?highlight=transparent#the-transparent-representation)
        unsafe { &*(r#struct as *const StructChunked as *const UnsaturatedChunked) }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn index(&self) -> PolarsResult<UInt8Chunked> {
        Ok(self.0.field_by_name(INDEX)?.u8()?.clone())
    }

    pub fn parity(&self) -> PolarsResult<BooleanChunked> {
        Ok(self.0.field_by_name(PARITY)?.bool()?.clone())
    }

    pub fn triple(&self) -> PolarsResult<BooleanChunked> {
        Ok(self.0.field_by_name(TRIPLE)?.bool()?.clone())
    }

    #[inline]
    pub fn fields(
        &self,
    ) -> PolarsResult<Unsaturated<UInt8Chunked, BooleanChunked, BooleanChunked>> {
        Ok(Unsaturated {
            index: self.index()?,
            triple: self.triple()?,
            parity: self.parity()?,
        })
    }
}

impl Unsaturated<UInt8Chunked, BooleanChunked, BooleanChunked> {
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = Unsaturated<Option<u8>, Option<bool>, Option<bool>>> {
        self.index
            .into_iter()
            .zip(self.triple.into_iter())
            .zip(self.parity.into_iter())
            .map(|((index, triple), parity)| Unsaturated {
                index,
                triple,
                parity,
            })
    }
}
