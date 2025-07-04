use super::Identifier;
use crate::prelude::*;
use polars::prelude::{enum_::EnumChunkedBuilder, *};
use std::{
    fmt::{self, Debug, Display, Formatter},
    mem::take,
    sync::LazyLock,
};

/// The triple bound chunked array.
#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct TripleBoundListChunked(pub(crate) ListChunked);

impl TripleBoundListChunked {
    pub fn new(list: ListChunked) -> Self {
        Self::try_new(list).unwrap()
    }

    /// Creates a new bound chunked array from a [`Series`].
    ///
    /// # Errors
    ///
    /// Returns an error if the series datatype is not the [`BOUND_DATA_TYPE`].
    pub fn try_new(list: ListChunked) -> PolarsResult<Self> {
        check_data_type(&list)?;
        Ok(Self(list))
    }

    #[inline]
    pub fn as_list(&self) -> &ListChunked {
        &self.0
    }

    #[inline]
    pub fn into_list(self) -> ListChunked {
        self.0
    }

    pub fn get(&self, index: usize) -> PolarsResult<Option<Int8Chunked>> {
        let Some(series) = self.0.get_as_series(index) else {
            return Ok(None);
        };
        Ok(Some(series.i8()?.clone()))
    }

    pub fn iter(&self) -> impl Iterator<Item = PolarsResult<Option<Int8Chunked>>> {
        self.0.amortized_iter().map(|series| {
            let Some(series) = series else {
                return Ok(None);
            };
            Ok(Some(series.as_ref().i8()?.clone()))
        })
    }

    pub fn pop(&mut self) {
        *self = Self(self.0.slice(0, self.0.len().saturating_sub(1)));
    }

    // pub fn push(&mut self) -> PolarsResult<()> {
    //     self.0
    //         .append_owned(Int8Chunked::full(PlSmallStr::EMPTY, 0, 1))?;
    //     let mut physical = take(self).0.into_physical();
    //     // SAFETY: we extend the physical by one `S` so we are still in bounds.
    //     let categorical = unsafe {
    //         CategoricalChunked::from_cats_and_dtype_unchecked(
    //             physical,
    //             TRIPLE_BOUND_DATA_TYPE.clone(),
    //         )
    //     };
    //     *self = Self(categorical);
    //     Ok(())
    // }

    pub fn sort(self, descending: bool) -> Self {
        Self(self.0.sort(descending))
    }

    pub fn with_name(self, name: PlSmallStr) -> Self {
        Self(self.0.with_name(name))
    }
}

// Mask.
impl TripleBoundListChunked {
    #[inline]
    pub fn map<T>(
        &self,
        f: impl Fn(&Int8Chunked) -> T,
    ) -> impl Iterator<Item = PolarsResult<Option<T>>> {
        self.0.amortized_iter().map(move |item| {
            let Some(series) = item else {
                return Ok(None);
            };
            Ok(Some(f(series.as_ref().i8()?)))
        })
    }

    #[inline]
    pub fn mask(&self, f: impl Fn(&Int8Chunked) -> bool) -> PolarsResult<BooleanChunked> {
        self.map(f).collect()
    }
}

impl TripleBoundListChunked {
    /// Returns the number of sized bounds in the fatty acid.
    pub fn bounds(&self) -> u8 {
        self.0.len() as _
    }

    /// Returns the total number of unsaturations in the bound chunked array.
    pub fn unsaturation(&self) -> u8 {
        self.bounds() * 3
    }
}

unsafe impl IntoSeries for TripleBoundListChunked {
    fn into_series(self) -> Series {
        self.0.into_series()
    }
}

// impl Display for TripleBoundListChunked {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         let mut iter = self.iter();
//         if let Some(Ok(Some(bound))) = iter.next() {
//             Display::fmt(&bound, f)?;
//             for bound in iter {
//                 Display::fmt(&bound, f)?;
//             }
//         }
//         Ok(())
//     }
// }

impl IntoIterator for &TripleBoundListChunked {
    type Item = PolarsResult<Option<Int8Chunked>>;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> TryFrom<&'a ListChunked> for &'a TripleBoundListChunked {
    type Error = PolarsError;

    fn try_from(value: &'a ListChunked) -> Result<Self, Self::Error> {
        check_data_type(value)?;
        // [safe](https://doc.rust-lang.org/reference/type-layout.html?highlight=transparent#the-transparent-representation)
        Ok(unsafe { &*(value as *const ListChunked as *const TripleBoundListChunked) })
    }
}

impl<'a> TryFrom<&'a Series> for &'a TripleBoundListChunked {
    type Error = PolarsError;

    fn try_from(value: &'a Series) -> Result<Self, Self::Error> {
        value.list()?.try_into()
    }
}

#[cfg(feature = "atomic")]
impl Atomic for &TripleBoundListChunked {
    type Output = u8;

    #[inline]
    fn carbons(self) -> u8 {
        self.bounds() + 1
    }

    #[inline]
    fn hydrogens(self) -> u8 {
        2 * self.carbons() - 2 * self.unsaturation()
    }
}

#[cfg(feature = "ecn")]
impl EquivalentCarbonNumber for &TripleBoundListChunked {
    type Output = u8;

    #[inline]
    fn equivalent_carbon_number(self) -> u8 {
        self.carbons() - 2 * self.unsaturation()
    }
}

/// Triple bound chunked array.
#[derive(Clone, Debug)]
pub struct TripleBoundChunked {
    pub index: Int8Chunked,
}

fn check_data_type(list: &ListChunked) -> PolarsResult<()> {
    polars_ensure!(
        *list.inner_dtype() == DataType::Int8,
        SchemaMismatch: "invalid triple bound list inner data type: expected `DataType::Int8`, got = `{}`",
        list.inner_dtype(),
    );
    Ok(())
}
