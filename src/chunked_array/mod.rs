pub use self::{fatty_acid::FattyAcidChunked, triacylglycerol::TriacylglycerolChunked};

use crate::prelude::*;
use polars::prelude::*;

/// Indices Chunked
#[repr(transparent)]
pub struct IndicesChunked(StructChunked);

impl IndicesChunked {
    fn new(r#struct: &StructChunked) -> &Self {
        // [safe](https://doc.rust-lang.org/reference/type-layout.html?highlight=transparent#the-transparent-representation)
        unsafe { &*(r#struct as *const StructChunked as *const IndicesChunked) }
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
}

mod fatty_acid;
mod triacylglycerol;
