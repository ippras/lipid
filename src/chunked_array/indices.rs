use crate::prelude::*;
use polars::prelude::*;

/// Indices Chunked
#[repr(transparent)]
pub struct IndicesChunked(StructChunked);

impl IndicesChunked {
    pub(super) fn new(r#struct: &StructChunked) -> &Self {
        // [safe](https://doc.rust-lang.org/reference/type-layout.html?highlight=transparent#the-transparent-representation)
        unsafe { &*(r#struct as *const StructChunked as *const IndicesChunked) }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn index(&self) -> PolarsResult<Int8Chunked> {
        Ok(self.0.field_by_name(INDEX)?.i8()?.clone())
    }

    pub fn parity(&self) -> PolarsResult<BooleanChunked> {
        Ok(self.0.field_by_name(PARITY)?.bool()?.clone())
    }

    pub fn triple(&self) -> PolarsResult<BooleanChunked> {
        Ok(self.0.field_by_name(TRIPLE)?.bool()?.clone())
    }

    #[inline]
    pub fn fields(&self) -> PolarsResult<Index<Int8Chunked, BooleanChunked, BooleanChunked>> {
        Ok(Index {
            index: self.index()?,
            triple: self.triple()?,
            parity: self.parity()?,
        })
    }
}

impl Index<Int8Chunked, BooleanChunked, BooleanChunked> {
    pub fn iter(&self) -> impl Iterator<Item = Index<Option<i8>, Option<bool>, Option<bool>>> {
        self.index
            .into_iter()
            .zip(&self.triple)
            .zip(&self.parity)
            .map(|((index, triple), parity)| Index {
                index,
                triple,
                parity,
            })
    }
}
