use polars::prelude::*;
use std::{fmt::Debug, num::NonZeroI8, sync::LazyLock};

pub const INDEX: &str = "Index";
pub const PARITY: &str = "Parity";

/// The double bound data type.
pub const DOUBLE_BOUND_DATA_TYPE: LazyLock<DataType> = LazyLock::new(|| {
    DataType::Struct(vec![
        Field::new(PlSmallStr::from_static(INDEX), DataType::Int8),
        Field::new(PlSmallStr::from_static(PARITY), DataType::Boolean),
    ])
});

/// Double bound list chunked array
///
/// This struct represents a chunked array of double bounds, which is a wrapper
/// around a [`ListChunked`]. It provides methods to interact with and
/// manipulate the double bound data.
#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct DoubleBoundListChunked(ListChunked);

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
