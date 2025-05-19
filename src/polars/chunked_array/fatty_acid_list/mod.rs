use std::{convert::identity, num::NonZeroI8};

use crate::prelude::*;
use polars::prelude::*;

/// Fatty acid list chunked array
///
/// This struct represents a chunked array of fatty acids, which is a wrapper
/// around a [`ListChunked`]. It provides methods to interact with and
/// manipulate the fatty acid data.
#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct FattyAcidListChunked(ListChunked);

impl FattyAcidListChunked {
    pub fn new(list: ListChunked) -> Self {
        Self::try_new(list).unwrap()
    }

    /// Creates a new [`FattyAcidListChunked`] from a [`ListChunked`].
    ///
    /// Ensures that the inner data type of the series is [`BOUND_DATA_TYPE`].
    ///
    /// # Arguments
    ///
    /// * `series` - A reference to a [`Series`] containing the fatty acid data.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a reference to the new
    /// [`FattyAcidChunked`].
    ///
    /// # Errors
    ///
    /// Returns a [`PolarsError`] if the inner data type of the series is not
    /// [`BOUND_DATA_TYPE`].
    pub fn try_new(list: ListChunked) -> PolarsResult<Self> {
        check_data_type(&list)?;
        Ok(Self(list))
    }

    pub fn as_list(&self) -> &ListChunked {
        &self.0
    }

    pub fn into_list(self) -> ListChunked {
        self.0
    }

    /// Retrieves the bound series at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the fatty acid in the chunked array to
    ///   retrieve.
    ///
    /// # Returns
    ///
    /// An [`Option`] containing the [`Series`] at the specified index, or
    /// [`None`] if the series does not contain the specified index
    pub fn get(&self, index: usize) -> Option<FattyAcidChunked> {
        Some(self.0.get_as_series(index)?.fatty_acid())
    }

    pub fn iter(&self) -> impl Iterator<Item = FattyAcidChunked> {
        self.0.amortized_iter().map(|series| {
            series.map_or_else(Default::default, |series| series.as_ref().fatty_acid())
        })
    }
}

// Mask.
impl FattyAcidListChunked {
    /// Applies a mask function to the chunked array.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes a reference to a [`BoundChunked`] and
    ///   returns a boolean.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with the result of
    /// the mask function.
    #[inline]
    pub fn mask(&self, f: impl Fn(FattyAcidChunked) -> bool) -> PolarsResult<BooleanChunked> {
        self.0
            .into_iter()
            .map(move |item| {
                let Some(series) = item else {
                    return Ok(None);
                };
                Ok(Some(f(series.try_fatty_acid()?)))
            })
            .collect()
    }

    /// Returns a boolean chunked array indicating which fatty acids are
    /// saturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// saturated fatty acids and [`false`] otherwise.
    #[inline]
    pub fn is_saturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|fatty_acid| fatty_acid.is_saturated().is_some_and(identity))
    }

    /// Returns a boolean chunked array indicating which fatty acids are
    /// unsaturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// unsaturated fatty acids and [`false`] otherwise.
    #[inline]
    pub fn is_unsaturated(&self, offset: Option<NonZeroI8>) -> PolarsResult<BooleanChunked> {
        self.mask(|fatty_acid| fatty_acid.is_unsaturated(offset).is_some_and(identity))
    }

    /// Returns a boolean chunked array indicating which fatty acids are
    /// monounsaturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// monounsaturated fatty acids and [`false`] otherwise.
    #[inline]
    pub fn is_monounsaturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|fatty_acid| fatty_acid.is_monounsaturated().is_some_and(identity))
    }

    /// Returns a boolean chunked array indicating which fatty acids are
    /// polyunsaturated.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// polyunsaturated fatty acids and [`false`] otherwise.
    #[inline]
    pub fn is_polyunsaturated(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|fatty_acid| fatty_acid.is_polyunsaturated().is_some_and(identity))
    }

    /// Returns a boolean chunked array indicating which fatty acids are cis.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// fatty acids with unsaturated cis-only bonds and [`false`] otherwise.
    #[inline]
    pub fn is_cis(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|fatty_acid| fatty_acid.is_cis().is_some_and(identity))
    }

    /// Returns a boolean chunked array indicating which fatty acids are trans.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`BooleanChunked`] with [`true`] for
    /// fatty acids with trans bonds and [`false`] otherwise.
    #[inline]
    pub fn is_trans(&self) -> PolarsResult<BooleanChunked> {
        self.mask(|fatty_acid| fatty_acid.is_trans().is_some_and(identity))
    }
}

unsafe impl IntoSeries for FattyAcidListChunked {
    fn into_series(self) -> Series {
        self.0.into_series()
    }
}

impl FromIterator<Option<Series>> for FattyAcidListChunked {
    fn from_iter<T: IntoIterator<Item = Option<Series>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for &FattyAcidListChunked {
    type Item = FattyAcidChunked;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> TryFrom<&'a ListChunked> for &'a FattyAcidListChunked {
    type Error = PolarsError;

    fn try_from(value: &'a ListChunked) -> Result<Self, Self::Error> {
        check_data_type(value)?;
        // [safe](https://doc.rust-lang.org/reference/type-layout.html?highlight=transparent#the-transparent-representation)
        Ok(unsafe { &*(value as *const ListChunked as *const FattyAcidListChunked) })
    }
}

impl<'a> TryFrom<&'a Series> for &'a FattyAcidListChunked {
    type Error = PolarsError;

    fn try_from(value: &'a Series) -> Result<Self, Self::Error> {
        value.list()?.try_into()
    }
}

#[cfg(feature = "atomic")]
impl Atomic for &FattyAcidListChunked {
    type Output = PolarsResult<UInt8Chunked>;

    #[inline]
    fn carbons(self) -> PolarsResult<UInt8Chunked> {
        self.map(|fatty_acid| fatty_acid.carbons()).collect()
    }

    #[inline]
    fn hydrogens(self) -> PolarsResult<UInt8Chunked> {
        self.map(|fatty_acid| fatty_acid.hydrogens()).collect()
    }
}

#[cfg(feature = "ecn")]
impl EquivalentCarbonNumber for &FattyAcidListChunked {
    type Output = PolarsResult<UInt8Chunked>;

    #[inline]
    fn equivalent_carbon_number(self) -> PolarsResult<UInt8Chunked> {
        self.map(|fatty_acid| fatty_acid.equivalent_carbon_number())
            .collect()
    }
}

fn check_data_type(list: &ListChunked) -> PolarsResult<()> {
    polars_ensure!(
        *list.inner_dtype() == *FATTY_ACID_DATA_TYPE,
        SchemaMismatch: "invalid fatty acid list inner data type: expected `FATTY_ACID_DATA_TYPE`, got = `{}`",
        list.inner_dtype(),
    );
    Ok(())
}

#[cfg(feature = "map")]
mod map;
#[cfg(feature = "select")]
mod select;
