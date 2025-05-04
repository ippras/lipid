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
    pub fn try_new(list: &ListChunked) -> PolarsResult<&Self> {
        polars_ensure!(
            *list.inner_dtype() == *FATTY_ACID_DATA_TYPE,
            SchemaMismatch: "invalid fatty acid list inner datatype: expected `{}`, got = `{}`",
            *FATTY_ACID_DATA_TYPE,
            list.inner_dtype(),
        );
        // [safe](https://doc.rust-lang.org/reference/type-layout.html?highlight=transparent#the-transparent-representation)
        Ok(unsafe { &*(list as *const ListChunked as *const Self) })
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

    pub fn into_list(self) -> ListChunked {
        self.0
    }
}

// unsafe impl IntoSeries for IndexedIdentifierListChunked {
//     fn into_series(self) -> Series {
//         self.0.into_series()
//     }
// }

// impl Deref for IndexedIdentifierListChunked {
//     type Target = ListChunked;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

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

impl<'a> TryFrom<&'a Series> for &'a FattyAcidListChunked {
    type Error = PolarsError;

    fn try_from(value: &'a Series) -> Result<Self, Self::Error> {
        FattyAcidListChunked::try_new(value.list()?)
    }
}

impl<'a> TryFrom<&'a ListChunked> for &'a FattyAcidListChunked {
    type Error = PolarsError;

    fn try_from(value: &'a ListChunked) -> Result<Self, Self::Error> {
        FattyAcidListChunked::try_new(value)
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

#[cfg(feature = "map")]
mod map;
#[cfg(feature = "mask")]
mod mask;
#[cfg(feature = "select")]
mod select;
